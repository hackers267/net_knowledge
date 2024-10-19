{{#title MessageChannel知识}}

# MessageChannel

> 备注:**MessageChannel**可在*Web Worker*中使用。

`Channel Message API`的*MessageChannel*接口允许我们创建一个新的消息通道，并通过它的两个 [MessagePort](./message_port.md) 属性发送数据。

## 构造函数

返回一个新的*MessageChannel*对象，其中包含两个新的 [MessagePort](./message_port.md "MessagePort") 

## 实例属性

*MessageChannel.port1*

返回channel的port1.

*MessageChannel.port2*

返回channel的port2.

示例:

```js
const channer = new MessageChannel();
const output = document.querySelector(".output");
const iframe = document.querySelect("iframe");

iframe.addEventListener("load",onLoad);

function onLoad() {
  channel.port1.onmessage = onMessage;

  iframe.contentWindow.postMessage("来自主页的您好！","*",[channel.port2]);
}

function onMessage(e) {
  output.innerHtml = e.data;
}
```


