export type Version = number
export const versionDef: Version = 0 //默认最小版本
export enum Status {
    Unpublished = 0, // 未发布
    Online = 1, // 正常线上可用
    Deprecated = 2, // 发布过但已被废弃
}
export enum Method {
    Get = 0,
    Post = 1
}
export enum ParameterType {
    Str = 0,
    Num = 1,
    NumLong = 2,
    NumDoube = 3,
    Josn = 4,
    Array = 5
}

export class ApiDoc {
    projectPath!: string // 项目json文件路径
    projectName?: string // 项目名
    api?: Api // 项目api内容
    version?: Version  //文件对应的版本
    info?: Info
}

export class Info {
    createTime!: number // 创建时间戳
    minVersion: Version = versionDef // 可用的最小版本(包含)
    maxVersion?: Version = undefined // 可用的最大版本(包含)
    status: Status = Status.Unpublished // 当前状态 
}

export class Api {
    path!: string // api路径, 只是指此部分的路径, 完整路径会加上其父类的路径及分隔符
    name: string = this.path //api名称
    method?: Method //方法类型, 如果时分类路径, 此值未定义
    version: Version = versionDef //api对应的版本
    info?: Info // api info
    headers?: string[] // 该api需要的header, 会联合其父路径的headers
    paramenter?: Parameter[] //  请求的参数

    response?: Parameter[] // 返回的参数

    children?: Api //拓展路径
}

export class Parameter {
    name!: string //参数名称
    type!: ParameterType //参数类型, 
    opetional: boolean = false //是否是可选参数
    default?: string //如果该值可选且未传入, 其会被视为的默认值
    exp?: string //示例值
}