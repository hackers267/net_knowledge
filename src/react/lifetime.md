# 生命周期

React组件的生命周期指的是组件从创建到销毁的整修过程，这个过程包括了几个关键的阶段:创建、更新和销毁。在不同的阶段，React提供了不同的生命周期方法，允许开发者在这些特定的时刻执行代码。

对于类组件，React17版本之前定义了以下生命周期方法:

1. **挂载阶段**
  - __contructor()__: 构建函数，用于初始化组件实例，如绑定方法和初始化状态。
  - __render()__: 渲染方法，返回要渲染的元素，这个方法是纯的，不应该执行与渲染无关的操作。如数据获取或副作用。
  - __componentDidMount()__: 组件挂载后(即添加到DOM树中)被调用。通常用于执行DOM操作、数据获取或其他需要在客户端完成的操作。
2. **更新阶段**
  - __render()__: 当组件的状态或属性改变时，render方法会被调用。
  - __shouldComponentUpdate()__: 在组件接收到新的属性或状态时被调用，返回一个布尔值，用于决定是可继续执行render方法。这是优化性能的地方。
  - __componentWillReceiveProps()__: 在组件即将接收到属性之前被调用。(已在React16.3中被弃用)
  - __componentWillUpdate()__: 在render方法执行之前被调用，最后一次渲染之前。
  - __componentDidUpdate()__: 在组件更新后被调用，可以在这里执行DOM操作或网络请求。
3. **卸载阶段**
  - __componentWillUnmount()__: 在组件卸载和销毁之前被调用，用于执行清理操作，如取消计时器、取消网络请求、清理在componentDidMount中设置的订阅等。

React16.3引入了新的生命周期方法，以替换旧的不安全的生命周期方法:
  - __static getDerivedStateFromProps()__: 在渲染之前和接收新属性时被调用，用于根据属性变化来更新状态。
  - __getSnapshotBeforeUpdate()__: 在更新之前(在render之后)被调用，允许在更新发生之前进行最后的修改，如捕获滚动位置或焦点状态等。

React17版本中，以下旧的生命周期方法被标记为废弃，并被新的生命周期方法所取代:
  - __componentWillMount()__: 在挂载之前被调用，已被*contructor()*和*static getDerivedStateFromProps()*所取代
  - __componentWillReceiveProps()__: 在接收到新的属性之前被调用，已被*static getDerivedStateFromProps()*所取代
  - __componentWillUpdate__: 在更新之前被调用，已被*getSnapshotBeforeUpdate()*所取代

**旧生命周期**:

![旧生命周期](./images/old_lifetime.webp "旧生命周期")

**新生命周期**:

![新生命周期](./images/new_lifetime.webp "新生命周期")

# 函数组件和类组件在生命周期上有什么不同?

函数组件和类组件在React中都是用来构建用户界面的，但它们在生命周期管理和功能实现上有一些关键的不同，以下是主要的区别:

1. __定义方式不同__ 
  - __类组件__: 使用ES6类或旧的构造函数来定义，可以包含生命周期方法、状态和上下文等。
  - __函数组件__: 使用JavaScript函数来定义，更简洁。
2. __生命周期方法不同__ 
  - __类组件__: 有明确的生命周期方法，如*render()*,*componentWillUnmount()*等
  - __函数组件__: 没有生命周期方法，但可以通过React Hooks(如useEffect)来实现和生命周期相似的行为。
3. __状态管理__
  - __类组件__: 使用*this.state*和*this.setState*来管理状态
  - __函数组件__: 使用*useState* Hook来管理状态
4. __上下文消费__ 
  - __类组件__: 通过*this.context*来访问上下文
  - __函数组件__: 通过*useContext* Hook来访问上下文
5. __性能优化__  
  - __类组件__: 通过*shouldComponentUpdate,getDerivedStateFromProps*等生命周期方法来进行性能优化
  - __函数组件__: 通过*React.meno*和*useMemo,useCallback* Hook来进行性能优化
6. __代码组织__ 
  - __类组件__: 通过需要更多的样板代码，如构造函数，render方法等。
  - __函数组件__: 代码通常更简洁，更容易组织和重用
7. __继承和组合__   
  - __类组件__: 支持继承，可以通过继承来复用代码 
  - __函数组件__: 不支持继承，但可以通过组合或自定义Hooks来复用代码
8. __Hooks__  
  - __类组件__: 不能直接使用Hooks
  - __函数组件__: 可以使用Hooks，如*useEffect*,*useState*等
9. __更新机制__   
  - __类组件__: 在状态或属性变化时，会触发render方法，但需要手动调用*this.setState*来触发更新
  - __函数组件__: 在状态变化或依赖项变化时，会自动重新渲染，通过*useState* 设置状态会自动触发组件的重新渲染。  
10. __实例化__
  - __类组件__: 每次渲染都会创建一个新的类组件实例。
  - __函数组件__: 每次渲染都会调用函数，但不是每次都创建一个新的实例。除非使用了useRef或类组件的引用。 
