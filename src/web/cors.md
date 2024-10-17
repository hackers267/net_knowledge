{{#title Web-跨域}}
# 跨域

**同源**:所谓的同源是指地址的**协议**,**主机**和**端口**完全一致。只要**协议**,**主机**和**端口**有一个不同，那么就是不同源。浏览器因为完全原因，有一个同源策略，只要不是同源的**内容脚本**，那就可能产生跨域问题。

**同源策略(SOP)**:核心是确保不同源的文件是相若独立的。默认情况下，XHR对象只能访问与包含它的页面处理同一域中的内容。这种限制可以预防某种恶意攻击(?什么攻击),但同时也有许多不便。

# 常见跨域问题的解决方案

- document.domain+iframe(子域名代理)
- jsonp实现跨域
- postMessage实现跨域
- CORS服务端解决跨域

## document.domain+iframe

对于主域名相同，子域名不同的情况。可以通过设置*document.domain*的方法来解决跨域问题。如对于两个文件'http://www.a.com/a.html'和'http://blog.a.com/b.html'均加上`document.domain=a.com`;然后在*a.html*文件中创建一个iframe，通过iframe两个js文件即可交互数据。

```html
<!-- www.a.com/a.html -->
<script>
document.domain = "a.com";
const iframe = document.createElement('iframe');
iframe.src = "http://blog.a.com/b.html";
document.body.appendChild(iframe);
iframe.onload = function() {
  var doc = iframe.contentDocument || iframe.contentWindow.document;
  // 在这里操作b.html
}
</script>
```

```html
<!-- blog.a.com/b.html -->
<script>
  document.domain = "a.com";
</script>
```

> 注:某一页面的*domain*默认等于`window.location.hostname`。主域名是不带*www*的域名，例如*a.com*；主域名前带有前缀的通常为二级域名或多级域名，例如*blog.a.com*就是二级域名。*domain*只能设置为主域名，不可以设置二级域名或多级域名。即不能在*blog.a.com*中设置*domain*为*www.a.com*.

## 服务器端的跨域解决方案

跨域资源共享(CORS) 定义了浏览器和服务器如何通过可控方式进行跨域通信。CORS通过添加特殊HTTP头信息以允许浏览器和服务器判断请求是成功还是失败。几乎所有现代浏览器都支持CORS。

当跨域发送HTTP请求时，支持CORS的浏览器会通过添加额外*Origin头信息*指定请求源，其值包括请求*协议*,*域名*,*端口*.

若请求头中无*Origin*头信息，服务器将不返回任何CORS头信息。

服务器接收到请求时会检查头信息屋宇是否接受请求：若接受请求，必须返回一个包含**Access-Control-Allow-Origin**响应头，其值与请求头*Origin*值相同，如:

```http
Access-Control-Allow-Origin: http://example.com
```

对于公共资源且允许任何源请求的数据，服务器通常返回该值为通配符 __*__,如:
```http
Access-Control-Allow-Origin: *
```

浏览器接收到服务器HTTP响应时，首先会检查*Access-Control-Allow-Origin*的值，只有当值存在且同*Origin*匹配，才继续处理该请求。

### Cookie及HTTP认证头

我们还需要知道不同于首钢的HTTP请求，使用CORS发道请求时，默认情况下，浏览器不会携带任何*Cookie*和*HTTP认证头*等识别信息。只有当我们将*XMLHttpRequest*对象的*withCredentials*属性值设为*true*，才在该请求发送时添加额外识别信息。

若服务器需要识别信息，则在响应头中添加*Access-Control-Allow-Credentials*响应头:

```http
Access-Control-Allow-Credentials: true
```

若发送请求时将*withCredentials*设为*true*，服务端没有返回该响应头，浏览器将拒绝该响应。

本来一切都是美好的，然而*XDomainRequest*不支持*withCredentials*属性。于是IE8,IE9并不支持请求时包含识别信息。

### 预检请求

使用CORS时，若请求方法不是GET,POST或HEAD，又或者使用了自定义的HTTP头，浏览器将发起预检请求。

预检请求是一个服务端认证机制，在执行请求之前会判断该请求是否合法。

发送复杂请求时，浏览器会将原始请求的方法和请求头作为预检请求的信息发送给服务器；服务器需要决定是否接受该请求并响应，预检请求通常是使用一种OPTIONS的HTTP方法。

客户端发起复杂请求时，会先发起预检，携带以下头信息:

请求头 | 描述
---|---
Origin | 请求源
Access-Control-Request-Method | 请求方法(HTTP方法)
Access-Control-Request-Headers | 请求自定义头(以逗号分隔)

服务器返回的响应头:

响应头 | 描述
---|---
Access-Control-Allow-Origin | 允许的请求源(与请求头Origin匹配)
Access-Control-Allow-Methods | 允许的请求方法(以逗号分隔)
Access-Control-Allow-Headers | 允许的请求头信息(以逗号分隔)
Access-Control-Allow-Credentials | 指定请求是否支持认证信息(可选)
Access-Control-Max-Age | 预检请求的缓存时长(单位:秒)

客户端接收到服务端的响应后，会使用之前声明的HTTP方法和请求头发送真正的请求。

预检请求会被浏览器缓存，缓存时间为*Access-Control-Max-Age*值，缓存时间内，后续相同类型的请求不再发起重复的预检请求。IE8,IE9不支持预检请求。

### HTML5 postMessage

HTML5的window.postMessage为浏览器带来了一个安全的,基于事件的消息API.与之前的子域名代理通过iframe跨子域通信不同，使用postMessage不再是直接访问一个文档的属性和方法，而是向文档发送消息然后等待响应，这要求形成一条双向的通信通道。

### JSONP实现跨域

JSONP,即带填充的JSON,可以在JavaScript中绕过同源策略，发起跨域HTTP请求。




