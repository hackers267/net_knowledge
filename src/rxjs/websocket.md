{{#title RxJS-WebSocket}}

# WebSocket

[webSocket](../../js/websocket.md "WebSocket简介")是一种在单位TCP连接上进行全双方通信的协议。它使得客户端和服务器之间可以进行实时的数据交互，而不需像传统的HTTP请求那样频繁地建立和断开连接。

RxJs也实现了对webSocket的封装和支持。

webSocket是一个生成`WebSocketSubject`的工厂函数，可用于与任意端点建立WebSocket连接。webSocket接受带有WebSocket端点url的字符串或`WebSocketSubjectCOnfig`对象作为参数，用于提供额外的配置，以及用于跟踪WebSocket连接的生命周期的Observer。

当WebSocjetSubject被订阅时，它会尝试建立一个Socket连接，除非已经建立了一个。这意味着许多订阅者将始终在同一个Socket上侦听，从而节省资源。但是，如果两个实例由WebSocketSubject组成，即使这两个实例提供了相同的url,它们也会尝试建立单独的连接。当WebSocketSubject的消费者退订时，只有在没有更多订阅者仍在监听的情况下，才会关闭Socket连接。如果一段时间后消费者再次订阅，则重新建立连接。

一旦建立连接，每当有新消息来自服务器时，WebSocketSubject都会将消息作为流中的值发出。默认情况下，来自Socket的消息会通过`JSON.parse`解析。如果你想自定义如何处理反序列化,你可以使用`webSocketSubjectConfig`中的`deserializer`处理。当连接关闭时，流将完成，前提是它没有任何错误发生。如果在任何时候(启动、维护或关闭连接)出现错误，无论WebSocket API抛出什么，流都会出错。

由于是Subject，webSocketSubject允许从服务器接收和发送消息。为了与连接的端点通信，请使用`next`、`error`和`complete`方法。next会向服务器发送一个值，请记住，该值不会事先序列化。因此，在使用某个结果调用next之前，必须手动对该值调用`JSON.stringify`。另外请注意，如果在下一个值抵达的时刻没有Socket连接(例如没有人订阅)，则这些值将被缓冲，并在最终建立连接时发送。complete方法关闭Socket连接。error也是如此，并通过状态代码和字符串通知服务器出现问题，并提供详细信息。由于WebSocket API需要状态码，因此WebSocketSubject不允许像常规Subject一样，将任意值传给error方法。需要使用带状态代码数字的code属性和带有描述错误详情的可选字符串reason的对象来调用它。

调用next不会影响WebSocketSubject的订阅者-它们没有任何信息表明某些内容已发送到服务器(当然，除非服务器以某种方式响应消息)。另一方面，由于调用complete会触发关闭Socket连接的尝试。如果该连接在没有任何错误的情况下关闭，则此流将完成，从而通知所有订阅者。而田由于调用error也会关闭Socket连接(只是服务器的状态码不同)，如果关闭本身没有错误，则订阅的Observable将不会出错(正如人们所期望的那样),但会正常完成。在这两种情况下(调用complete或error)，如果关闭Socket连接的过程导致一些错误，则流将出错。

## multiplex(多路复用)

WebSocketSubject有一个额外的操作符，在其它Subject中没有。它被称为multiplex,它用于模拟打开多个Socket连接，而实际上只真正维护一个。例如，一个应用程序既有聊天面板，也有关于体育新闻的实时通知。由于这是两个不同的功能，因此每个功能设置两个单独的连接是有意义的。也许甚至可以有两个具有WebSocket端点的单独服务，在单独的机器上运行，只有GUI将它们组合在一起。每位功能都有一个Socket连接可能会变得过于昂贵。将单个WebSocket端点用作其它服务(在本例中为聊天和体育新闻服务)的网关是一种常见模式。即使客户端应用程序中只有一个连接，也希望能够像处理两个单独的Socket一样操作流。这省去了在网关中手动注册和注销给定服务的工作，并过滤掉某些感兴趣的消息。这正是multiplex方法的用例。

方法授受三个参数。前两个分别是返回订阅和退订消息的函数。每当结果Observable的消费者订阅和退订时，这些消息都会发送到服务器。服务器可以使用它们来验证某种消息是应该开始还是停止转发给客户端。在上面的示例应用程序中，在获取具有适当标识符的订阅消息后，网关服务器决定它要连接到真空的体育新闻服务并开始从中转发消息。请注意，这两条消息都将作为函数返回的内容发送，默认情况下，它们会使用`JSON.stringify`进行序列化，就像通过next推送的消息一样。另外请记住，这些消息会在**每次**订阅和退订时发送。这是潜在的危险，因为Observable的一个消费者可能会退订，并且服务器可能会停止发送消息，因为它收到了退订消息。这需要在服务器上处理，或者在从"multiplex"返回的Observable上使用public.

multiplex的最后一个参数是一个messageFilter函数，它应该返回一个布尔值。它用于过滤服务器发送的消息。只发送给那些属于模拟WebSocket流的消息。例如，服务器可能会在消息对象上用某种字符串标识符标记这些消息，如果Socket发出的对象上有这样的标识符，则messageFilter将返回true。在messageFilter中返回false的消息将被简单地跳过，并且不会沿着流向下传递。

multiplex的返回值是一个Observable，其中包含从Socket连接传入的消息。请多端，这不是WebSocketSubject，因此再次调用next或multiplex将失败。要将值推送到服务器，请使用根WebSocketSubject.

## 示例

```javascript
import { webSocket } from "rxjs/websocket";

const subject = webSocket('ws://locahost:8081');
const observer = {
  next: msg => console.log("message receiver: " + msg),
  error: err => console.log(err),
  complete: () => console.log("complete")
};

subject.subscribe(observer);

subject.next("message from the RxJS webSocketSubject")
```
