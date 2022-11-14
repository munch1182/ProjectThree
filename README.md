[toc]

# tauri

官网: https://tauri.app/v1/guides/getting-started/setup

### 项目创建
- 创建前端ui文件夹和后端rust文件夹平级的项目结构, 项目使用`vite`+`vue`+`typescript`

##### 使用cargo创建和执行
1. 第一次需要使用`cargo install create-tauri-app`命令
2. 使用`cargo create-tauri-app`创建项目, 并选择`cargo`作为包管理,`vanilla`作为`UI`模板
3. 此时删除`src`文件夹, 使用`npm create vite@latest`来安装`vite`, 项目名称为`src`, 并选择`vue`+`typescript`
4. 修改`src-tauri/tauri.config.json`文件中的`build`部分
    ```
    "build": {
        "beforeDevCommand": "cd src & npm run dev",
        "beforeBuildCommand": "cd src & npm run build",
        "devPath": "http://127.0.0.1:5173",
        "distDir": "../src/dist",
        "withGlobalTauri": false
    },
    ```
    `before~Command`为`rust`代码执行前的命令, 因此`npm`在`src`目录下才能执行, 开发时需要先开启`vite`的服务, 编译时需要先编译前端代码
    `devPath`为开发环境下的资源路径前缀, 使用`vite`则应该为`vite`的开发服务器地址
    `distDir`为正式环境下的资源路径前缀, 使用`vite`则应该为`vite`的编译结果文件夹
    `withGlobalTauri`为`true`时可直接通过`window.__TAURI__`访问`tauri`, 使用通信来访问则设置`false`
5. 进入`src`文件夹, 使用`npm install`命令, 然后退回项目根目录,即可使用`cargo tauri`相关命令

此方法可使用`vite`和`vue-ts`最新版本, 并更好理解`tauri`的文件结构

##### 使用npm创建并使用cargo执行
1. 使用`npm create tauri-app`命令创建项目, 并选择`npm`作为包管理器,`vue-ts`作为`UI`模板
2. 将目录下除`src-tauri`(和`.vscode`)之外的所有文件移动到与`src-tauri`同级的新建文件夹`src`中
3. 修改`src-tauri/tauri.config.json`文件中的`build`下的`beforeDevCommand`和`beforeBuildCommand`的命令增加`cd src`, 即`cd src & npm run dev`和`cd src & npm run build`
4. 进入`src`文件夹, 使用`npm install`命令, 然后退回项目根目录,即可使用`cargo tauri`相关命令

此方法可自动创建前端`vite`和`vue-ts`相关代码

##### 创建问题
1. `npm`命令如遇网络问题可使用`cnpm`代替
`npm install cnpm -g --registry=https://registry.npmmirror.com`
2. `src/dist`文件夹需要自行创建, 否则会有错误提示
3. 创建完成后, `src`是一个单独前端项目, `src-tauri`是一个单独的`rust`项目, 两者之间通过`tauri`通信, 因此对应`npm`和`cargo`相关命令, 需要在各自文件夹下执行, 只有`cargo tauri`相关命令才能在根目录下执行
4. 需要修改的设置: `src-tauri/tauri.conf.json`中的`version`和`identifier`, `src/package.json`中的`version`和`name`, `src-tauri/Cargo.toml`中的`version`和`authors`, 这些不修改并不影响使用
5. 可使用`git clone -b tauri https://github.com/munch1182/Project.git`直接使用此模板创建(//todo, 名称+版本同步更改)

### 前端安装

##### Pinia

1. 在`src`路径下安装: `npm install pinia`
2. 使用:
    ```ts
    // src/src/main.ts
    ...
    import { createPinia } from 'pinia'
    ...
    createApp(App).use(createPinia()).mount('#app')
    ```
3. 官网: https://pinia.vuejs.org/zh/introduction.html
4. 参考: https://zhuanlan.zhihu.com/p/533233367


##### VueRouter

1. 在`src`路径下安装: `npm install vue-router@4`
2. 使用`vue-router`的`createRouter`方法创建`router`并交由`app`使用
3. 官网: https://router.vuejs.org/zh/guide/#router-view

##### Tailwindcss

1. 在`src`路径下安装: `npm install -D tailwindcss postcss autoprefixer`
2. 在`src`路径下执行`npx tailwindcss init -p`, 并在生成的`tailwind.config.cjs`文件中修改`content`部分为
    ```js
    content: [
        "./index.html",
        "./src/**/*.{vue,js,ts,jsx,tsx}",
    ],
    ```
3. 在基础`css`文件(如`src/src/style.css`)中引入
    ```css
    @tailwind base;
    @tailwind components;
    @tailwind utilities;
    ```
4. 官网: https://tailwindcss.com/docs/guides/vite#vue
5. 在`vue SFC`的`style`中使用时, 应设置`lang="postcss"`以使用`@apply`

### 自定义窗口

自定义窗口即不使用系统的窗口样式, 使用自定义的UI并实现窗口功能

1. 创建窗口并实现功能, 这部分使用`css`+`js`完成
    ```js
    import { appWindow, WebviewWindow } from "@tauri-apps/api/window";
    // appWindow即当前有焦点的窗口, 可使用其`minimize`、`close`及`setAlwaysOnTop`等方法控制窗口
    // WebviewWindow即窗口对象, 新建WebviewWindow即新建窗口, 其path即可直接使用`router`路径

    <div data-tauri-drag-region></div> // 为该div实现拖拽窗口、双击放大缩小窗口的功能
    ```
2. 取消窗口自定义
    对于在`src-tauri/tauri.conf.json`中配置的窗口, 配置`decorations: false`即可取消默认窗口样式
    对于通过`js`或者`rs`创建的窗口, 使用窗口对象`setDecorations(false)`即可
3. 恢复窗口阴影
    `winow`下取消窗口样式会同时取消窗口阴影, 要恢复阴影, 使用[`window-shadows`库](https://crates.io/crates/window-shadows), 并使用`tauri-plugin`为每一个创建的窗口都加上阴影
    ```rust
    use log::info;
    use tauri::{plugin::Plugin, Runtime, Window};

    pub struct TaruiWindowPlugin {}

    impl<R: Runtime> Plugin<R> for TaruiWindowPlugin {
        fn name(&self) -> &'static str {
            "tauri-plugin-window"
        }

        fn created(&mut self, window: Window<R>) {
            if let Err(_) = window_shadows::set_shadow(window, true) { // 为窗口加上阴影
                info!("error to shadow window.")
            }
        }
    }

    // 使用
    tauri::Builder::default()
        .plugin(TaruiWindowPlugin::new())
        .run(tauri::generate_context!())
    ```