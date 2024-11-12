---
title: "K8s Nginx Ingress Proxy Ssl"
weight: 1
---

# 如何配置K8s Nginx Ingress的Proxy SSL

## 背景

我们在部署K8s的服务中，有一个Pod是开源的框架，其后端接口要求必须配置SSL证书。
在不修改源代码的前提下（主要是为了向后兼容），我们尝试在Ingress层面对后端的Pods的SSL证书
进行兼容。
因此，我们在理顺相关的流程后，将相关的过程和经验撰写为本文。

## 证书生成

我们的开源后端有三个配置（排查很久后才发现的3，最开始只知晓1和2）：

- Cert：证书的公钥
- Key：证书的私钥
- CA：CA证书（证书颁发机构证书）

我们是通过如下命令生成的相关证书（自颁发）：

```shell
openssl req -x509 -nodes -newkey ec \
    -pkeyopt ec_paramgen_curve:secp521r1 \
    -pkeyopt ec_param_enc:named_curve  \
    -subj '/CN=eng.example.com' \
    -keyout ./tls-key.pem -out ./tls-cert.pem -sha256 -days 3650 \
    -addext "subjectAltName = DNS:localhost,DNS:example.com,DNS:*.example.com,DNS:eng.example.com,DNS:demo1.example-eng.svc.cluster.local" \
    -addext "extendedKeyUsage = serverAuth,clientAuth" \
    -addext "keyUsage = digitalSignature, keyCertSign, keyAgreement"
```

### 避坑点

#### 需要使用`clientAuth`

当作为Client（Ingress Controller的角色）的时候，如果你要使用相关的证书，是需要声明为`clientAuth`的。如若不然，则会出现报错。

#### 需要设置多个`subjectAltName`

因为我们在Pod中部署，同时在调试的时候，可能需要使用不同的Hostname（主机名）来测试Endpoint的可用性。
因此，我们需要设置多个`DNS`来确保我们可以使用这些地址，比如上述命令中的：

`localhost,example.com,*.example.com,eng.example.com`还有`demo1.example-eng.svc.cluster.local`

最后一个是我们在Pod中进行测试Service表示的Pod所用到的。

因此，通过设置多个DNS，我们可以使用不同的Hostname，来测试访问同一个服务。

## Nginx Ingress Controller Proxy SSL相关设置

我们的Kubernetes的Ingress设置如下：

```yaml
---
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: example-ingress
  labels:
    app: example
  annotations:
    nginx.ingress.kubernetes.io/backend-protocol: "HTTPS"
    nginx.ingress.kubernetes.io/ssl-verify: "true"
    nginx.ingress.kubernetes.io/proxy-ssl-secret: "example-eng/example-ingress-proxy-ssl"
    nginx.ingress.kubernetes.io/proxy-ssl-verify: "on"
    nginx.ingress.kubernetes.io/proxy-ssl-verify-depth: "1"
    nginx.ingress.kubernetes.io/proxy-ssl-name: "eng.example.com"
    nginx.ingress.kubernetes.io/proxy-ssl-server-name: "on"
spec:
  ingressClassName: nginx
  rules:
  - host: eng.example.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: example
            port:
              name: http
```

我们逐一来进行介绍：

- `nginx.ingress.kubernetes.io/backend-protocol: "HTTPS"`：表明，我们的后端（Pod）的接口是使用的HTTPS协议。
- `nginx.ingress.kubernetes.io/ssl-verify: "true"` 以及 `nginx.ingress.kubernetes.io/proxy-ssl-verify: "on"`：表示我们希望启用SSL的验证（Ingress Controller于后端Pod之间的网络流量认证。
- `nginx.ingress.kubernetes.io/proxy-ssl-secret: "example-eng/example-ingress-proxy-ssl"`： 这个是表明，我们的Ingress Controller所使用的相关证书对应的K8s Secret，格式为`namespace/secret-name`。（后面我们展开讲）
- `nginx.ingress.kubernetes.io/proxy-ssl-name: "eng.example.com"` 以及 `nginx.ingress.kubernetes.io/proxy-ssl-server-name: "on"`：表示，我们希望采用的后端的名称（类似上述证书中对应的DNS）。


### 生成secret

我们是通过下述代码来生成我们需要的密钥的：

```shell
kubectl create secret generic example-ingress-proxy-ssl \
    --from-file=tls.crt=tls-cert.pem \
    --from-file=tls.key=tls-key.pem \
    --from-file=ca.crt=tls-cert.pem \
    -n example-eng \
    --dry-run=client -o yaml | kubectl apply -f -
```

其中，`tls.crt`我们Client的公钥，`tls.key`是我们的私钥，`ca.crt`是我们信任的根证书（这里我们使用了和我们公钥同样的配置，因为我们自签发的证书）

### 小结

通过使用上面的配置，我们就告诉了 Ingress Controller，我们希望使用相关的配置，与我们的Pod后端进行SSL的认证。

## 问题排查

### 深入到Ingress Controller中

我们在排查问题的时候，可以深入到Ingress Controller的Pod中，默认在`nginx-ingress`命名空间中。

我们通过：

```shell
kubectl -n nginx-ingress get pods
```

上面的命令可以查看到我们目前的Ingress的情况，如结果：

```
NAME                                       READY   STATUS    RESTARTS   AGE
ingress-nginx-controller-9756f5bd9-vr9dd   1/1     Running   0          19d
```

在排查过程中，我们一方面可以通过`kubectl logs`命令来监控Ingress的Log输出：

```shell
kubectl -n nginx-ingress logs -f ingress-nginx-controller-9756f5bd9-vr9dd
```

另一方面，我们可以深入到Ingress Controller的Pod中。

### 深入到Pod中

通过命令：

```shell
kubectl -n nginx-ingress exec -it ingress-nginx-controller-9756f5bd9-vr9dd -- bash
```

我们可以登陆到我们的Pod中，然后我们检查一下我们的Secret的配置，默认secret在：
`/etc/ingress-controller/ssl`
这个文件夹中，如果我们配置正确，我们应该可以看到上面提到的secret：
`example-ingress-proxy-ssl`。

通过`vi`或者`cat`命令，我们可以检查我们之前配置的`tls.crt`等是否正确被加载到了Controller中。

### 检查Ingress Config

除了查看SSL的情况，我们也可以通过打开
`/etc/nginx/nginx.conf`
文件，确定我们的相关Ingress声明，是否有被正确的转换为Nginx配置：

```shell
cat /etc/nginx/nginx.conf | grep example-eng -A 100
```

我们可以看到相关输出：

```
proxy_ssl_trusted_certificate           /etc/ingress-controller/ssl/example-eng-example-ingress-proxy-ssl.pem;
proxy_ssl_ciphers                       DEFAULT;
proxy_ssl_protocols                     TLSv1.2;
proxy_ssl_verify                        on;
proxy_ssl_verify_depth                  1;

proxy_ssl_name                          eng.example.com;
proxy_ssl_server_name                   on;

proxy_ssl_certificate                   /etc/ingress-controller/ssl/example-eng-example-ingress-proxy-ssl.pem;
proxy_ssl_certificate_key               /etc/ingress-controller/ssl/example-eng-example-ingress-proxy-ssl.pem;
```

这说明，我们的配置是生效的，Nginx正在使用相关的配置来和我们的Pod服务器进行SSL的沟通。

### 测试访问

我们为了继续排查，可以在Ingress Controller中，对相关的后端服务进行测试，如下命令；

```shell
curl -v --tlsv1.3 \
    --cacert ./ca \
    --cert ./cert \
    --key ./key \
    -vvv \
    https://demo1.example-eng.svc.cluster.local:4443
```

请注意，上面的`./ca`, `./cert`以及`./key`等文件，需要创建好进行测试。

## 其他

由于我们最前端的CDN层，我们使用的是CloudFlare，因此，我们对外使用的是CloudFlare提供的SSL证书。
然后由其将流量转发给我们的后端Kubernetes集群，再由Ingress转发给我们的Pod，最终由Pod对外提供服务。

因此，在我们修复了上述问题后，我们可以顺利使用Cloudflare来访问我们的后端资源。
（如果SSL异常，我们大概率会看到502的报错信息）

## 参考

- [Backend Certificate Authentication](https://github.com/kubernetes/ingress-nginx/blob/main/docs/user-guide/nginx-configuration/annotations.md#backend-certificate-authentication)
- [nginx docs](https://nginx.org/en/docs/http/ngx_http_proxy_module.html#proxy_ssl_name)


## 总结

我们本文主要介绍了，如何配置使用Nginx Ingress Controller
来与必须兼顾SSL的Pod后端接口进行通信。
因为网上相关材料较少，我们整理出来以便大家参考。

