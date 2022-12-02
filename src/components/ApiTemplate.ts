export default `
# 通过设置保留参数可以进行相关设置
@_SHOW_MOCK_HEADER = true # 如果是mock的请求或者返回, 会默认在header中添加一个标记, 使用这个保留参数可以开关这个参数

# 默认定义
@BASEURL = "https://www.api.com/api/v1"  # 如果设置了这个值, 会自动拼凑请求的url
@HEADERS = { # 如果设置了这个值, 会给所有的请求添加这些header值
    FROM: APP
    TOKEN: @token # 如果设置的是变量, 则只有其有值的时候才会添加
}
@BASERES = { # 如果设置了这个值, 会自动拼凑返回的结果定义
    "code": u8,
    "data": * # 表示这个值的实现会被替换
}
@OK: @BASERES.code == 0 # 定义OK的判断方式

# 全局定义
@user = {
    "username": "testuser1"
}
# 定义无值的变量
@token: str
@userid: str
@userinfo: {
    "username": str,
    "anyinfo": str
}

# SET: 表示此方法会更新@token和@userid的值, 
# 此标记会将此方法注册为这两个值的更新请求(会注册所有的请求), 需要这两个值的任一一个且其没有值时,
# 就会调用此请求来尝试为其赋值
# 注意: SET只是表示要请求@token和@userid需要调用此方法, 如何设置这个值, 仍需要在OK中实现
### SET [@token, @userid] 

# 请求: 方法 url
GET @BASEURL/login # 可省略@BASEURL, 其如果有值, 会默认加上

# header
ContentType: application/json
x-header: any

{
    "username": str # 定义只有类型
}

=> # 分割请求和响应的定义

{ # 如果有BASERES, 则会默认在外部包裹BASERES, 也可以显示标注
    "userid": u16,
    "usertoken": str,
    "collect": [@collect]
}

@collect = { # 局部定义, 局部定义可以写在使用后面
    "name": str,
    "id": u16
} <= { # 对定义的变量进行mock, 未标注的部分使用默认MOCK
    "name": "g_p_" + random_zh(12) # random_zh 默认mock方法, 返回随机生成的12分中文字符
}

MOCK @user => { # mock使用=>前为请求后为响应 # 直接使用()进行内容替换
    "userid": 1000001 # 未标注的部分使用默认MOCK
}
OK { # 执行成功的操作
    @token = this.usertoken # this可以指代BASERES, 也可以指代BASERES内被替换的值, 会进行两次尝试(是否根据自动设置同名参数?)
    @userid = this.userid
}
ERR { # 执行失败的操作
    @token = null
}
###

# MUST: 如果要执行该请求, 则@token必须要有值, 如果没有, 会自动先执行为@token注册的设置请求(不包括此请求)
# SET: 执行完毕后, 又会更新@token的值, 如果有其它方法需要@token且@token无值时, 则会调用这两个请求
### MUST [@token] SET [@token] 

POST /update

=> { # 无参数请求(会自动在header中携带token), 只定义响应
    "ok": u8
    "token": str? # 可null返回, 如无此标记, 则默认返回不会为null
}

MOCK => { "ok": random(1,2) } # 只mock结果

@OK: this.ok == 0 # 重新单独定义OK的判断条件

OK { @token = this.token }
###
`