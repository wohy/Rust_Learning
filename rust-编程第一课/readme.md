# 将极客时间的专栏 转化为 gitbook
$ npm install -g gktimesc2gitbook
填写好对应的 gitbook.json，其中 cid 为当前专栏对应的 id ，专栏对应的 url 上有。cookie 为用户登录后的 cookie，需要现有专栏权限才可以下载
$ gktimesc2gitbook ./gitbook.json

# readme.md 转化为 gitbook /.epub / pdf
是有低版本的 node
这里使用 10.15.1
1. npm install -g gitbook-cli
2. gitbook init
## 转化为 gitbook html
在 book.json 所在路径下执行 gitbook build ./ ./output  即可构建出 一个 gitbook 在当前文件的 output 文件夹中。该文件下的 HTML 可以直接 live-server 执行，将展示一个 gitbook

## 转化为 .epub 会复杂一点
1. 先在下载 calibre https://calibre-ebook.com/
2. mac 控制台执行 sudo ln -s /Applications/calibre.app/Contents/MacOS/ebook-convert /usr/local/bin/ebook-convert
3. ebook-convert --version 查看是否命令生效了
4. 生效后 在 book.json 所在路径下执行 gitbook build ./ ./output.epub 等待一段时间，路径下将生成一个 output.epub 文件。（文件名可自定义）