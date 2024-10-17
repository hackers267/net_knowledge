{{#title MySQL-OrderBy}}

# OrderBy

MySQL中的order by子句用于根据指定的列查询结果集进行排序。以下是一些关于 order by的重要知识点:

**1. 基本用法**

- *order by*子句通常放在*where*子句之后，*limit*子句之前。
- 可以对一个或多个列进行排序

```sql
select column1, column2 from table_name order by column1;
```

**2. 升序和降序**

- 默认情况下，*order by*对列进行升序排序(*ASC*)
- 使用*desc*关键字可以指定降序排序

```sql
-- 升序
select * from users order by age;
select * from users order by age asc;
-- 降序
select * from users order by age desc;
```

**3. 排序表达式**

- 可以在*order by*中使用列名、计算表达式或列的位置(通过数字指定)

```sql
select name,age * 2 as double_age from users order by double_age;
```

**4. 多列排序**

- 可以同时对多个列进行排序，MySQL将首先按照第一个列进行排序，如果第一个列相同，则按照第二个列排序，以此类推。

```sql
select * from orders order by order_date asc, total_amount desc;
```

**5. 排序空值**

- 在MySQL中，*NULL*值被认为是最小的值。在升序排序中，*NULL*会排在最前面，在降序排序中，*NULL*会排在后面。

**6. 与LIMIT结合使用:**

- *order by*与*limit*子句结束使用时，可以获取排序后结果集的特定部分。例如获取N个记录或相信范围中的记录。

```sql
select * from employees order by salary desc limit 10;
```

**7. 使用别名**

- 如果列名在*select*列表中使用了别名，那么在*order by*子句中也应该使用别名。

```sql
select name as full_name, age from users order by full_name;
```

**8. 排序大结果集**

- 对于大型结果集，使用*order by*可能会导致性能问题，因为排序需要额外的资源。
- 如果在*order by*中使用的列上有索引，那么排序操作可能会更快。

**10. 与*group by*和*distinct*一起使用**

- 与*group by*和*distinct*时，*order by*通过在这些子句之后进行。

```sql
-- 与group by
select department,AVG(salary) as avg_salary from employees group by department order by avg_salary desc;

-- 与distinct
select distinct department from employeees order by department;
```

**11. 排序非选择列**

- 通常*order by*中使用的列应该是*seelct*语句中选择的列。如果需要挑在非选择列，可能需要调整查询策略。

**12. 排序结果的稳定性**

- 在某些情况下，排序结果的顺序是稳定的，这意味着相同值的行将按照它们在表中的原始顺序出现。但这并不是所有数据库系统都保证的。 

**13. 排序文本和数字**

- 当对计和数字进行排序时，MySQL会将数字作为文本的一部分进行比较。

**14. 排序浮点数**

- 对浮点数进行排序时，需要注意精度问题，因为浮点数可能存在精度误差。

# QA

- **MySQL中的排序使用的是什么算法?是否是稳定排序算法?**

在MySQL中的排序中主要使用了3种排序方法:

- 快速排序法
- 归并排序法
- 堆排序法

但并不是在所有的情况下都会用到这些排序方法。

当数据量比较小的时候，MySQL会使用*快速排序*算法对数据进行排序。但当数据量达到一定大小的时候，就是启用*归并排序*，那么这个大小是多大呢？这个大小就是*sort buffer*的大小。因为MySQL中排序是以以下的方法进行的:

1. 获取所有满足*where*条件的数据记录。
2. 对于每条记录，将记录的主键+排序键取出放入*sort buffer*中
3. 当
  - *sort buffer*可以存放所有记录的主键+排序键时，直接进行排序
  - *sort buffer*存放不下所有记录的主键，则会产生临时文件，临时文件中保存的是已排序好的记录。在产生临时文件后，会使用*归并排序*以确保临时文件中的记录是**有序**的
4. 循环上面的过程，直到所有记录全部排序完成
5. 扫描排序好的序列对，并利用*id*到获取select需要返回的列
6. 把扫描结果返回给用户

我们看到，在排序中，如果参与排序的结果集很大或参与排序的字段很多，就会很容易的达到*sort buffer*的上限，从而产生临时文件，而触发耗时的IO操作。同时，我们发现，我对所有的有关记录排序完成后，MySQL会重新使用id再去获取我们需要的列。如果我们在保证排序合理的同时，保证排序后的id字段也是有序的，那么我们就可以提升排序完成后，扫描返回列的性能，从而在整体上提升查询语句的性能。

那么是什么决定了*sort_buffer*的大小呢?是*sort_buffer_size*参数决定的。

更多详情请参见[order by 原理以及优化](https://cloud.tencent.com/developer/article/1181272)

在上面我们提到的3种排序算法中，会稳定性为:

算法 | 稳定性
---|---
快速排序 | 不稳定排序
归并排序 | 稳定排序
堆排序 | 不稳定排序



- **影响排序的内容有哪些?**


- **如何自定义排序?**

可以利用*order by*中可以使用函数和表达式的特性来实现自定义排序，比如以下的场景:

用户数据库中按姓名排序，排序按*赵,钱,孙,李*的顺序排序，如果是其它姓氏，则不需要特定排序。

```sql
select name,age,sex,
  case
    when name like '赵%' then 1
    when name like '钱%' then 2
    when name like '孙%' then 3
    when name like '李%' then 3
    else 5
  end as full_name
  from user
  order by full_name,name;
```

- **MySQL中关键字执行的优先级是怎样的**

```mermaid
flowchart LR
  where --> group("groub by") --> having --> select --> distinct --> order("order by") --> limit
```
