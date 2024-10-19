# Hooks

Hooks是React中函数式组件中管理状态的一种方式。Hooks是在React16.8中引入的一个特性。在引用Hooks后，我们就可以在不书写类组件的情况下创建有状态的组件了。也就是说引用的Hooks让函数式组件也可以拥有状态和生命周期等特性。可以以更加函数式的方式来书写React组件了。下面是常用的几个Hooks:

# 内置的Hooks

在React中为我们提供了以下的几种Hooks:

- [useState](#usestate)
- [useEffect](#useeffect)
- [useRef](#useref)
- [useContext](#usecontext)
- [useCallback](#usecallback)
- [useMemo](#usememo)
- [useLayoutEffect](#uselayouteffect)
- [useReducer](#usereducer)
- [useImperativeHandle](#useimperativehandle)


## useState

*useState* 是React的所有Hooks中使用频率最高的，没有之一。其也是最简单的一个。作用就是在在函数式组件为管理状态。其基本使用如下:

```jsx
function Demo() {
  const [count, setCount] = useState(0);
  return <div @click={()=>next()}>{count}</div>
  function next() {
    setCount(count=>count+1);
  }
}  
```

上面的示例是*useState*最常使用的方式之一。我们看到*useState*接收一个参数作为状态的初始值，并返回一个数组。其中，这个数组包含两个值，第一个值为*当前状态*,第二个值为一个函数。这个函数使用设置下一次渲染时的状态值。当我们调用了这个函数的时候，在下一次的渲染时，状态值就会更新。

返回的这个函数，其即可以接收一个值来作为下一次的状态值，也可以接收一个函数。把这个函数的返回值作为下一次的状态值。当这个函数接收一个函数时，接收的函数会得到一个参数，这个参数是当前状态的状态值。因此，我们可以根据这个函数中的这个参数来更新下一次的状态值。即可以在当前函数中根据当前状态来淡定下一次的状态。这样的作法更加函数式，而不必依赖*useState*返回的第一个值来更新下一个状态值。

## useEffect



## useRef


## useMemo

## useCallback


## useContext


## useReducer


## useImperativeHandle


## useLayoutEffect


# 自定义Hooks

# Hooks的使用限制
