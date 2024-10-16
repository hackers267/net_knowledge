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

- MySQL中的排序使用的是什么算法?是否是稳定排序算法?


- 影响排序的内容有哪些?


- 如何自定义排序?


