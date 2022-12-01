import { Monaco as M } from "@monaco-editor/loader";
import { editor, CancellationToken, languages } from 'monaco-editor';
type Monaco = M;

const NAME = 'api';
const THEME = 'apiTheme';

const keywords = ['GET', 'POST'];
const operators = ['=>', '=', '+', '|', '###'];
const typeKeywords = [
    'str', 'bool', 'num',
    'u8', 'u16', 'u32', 'u64',
    'i8', 'i16', 'i32', 'i64',
    'f16', 'f32'];

//todo类型关键字和操作符颜色
// [正则, token, color]
const colorDefine: [RegExp, string, string][] = [
    [/###/, 'split', '#ff3d00'], // 分隔符: 固定###
    [/#.*$/, 'comment', '#438a55'], // 注释: 以#开头直到结尾
    [/@([\w]*)/, 'define', '#3ac1f1'], // 定义变量: @开头直到非字符
    [/\b(GET|POST|MOCK-REQ|MOCK-RES)\b/, 'fun', '#004d99'], // 方法关键字
    [/"[^"]*"/, 'string', '#ce723b'], // 字符串: 以"开头以"结尾中间没有"的部分
]
const tokenRoot = colorDefine.map(i => { return [i[0], i[1]] as [RegExp, string] })
const colorRules = colorDefine.map(i => { return { token: i[1], foreground: i[2] } })

export default { register, addCommand, NAME, THEME };

function register(monaco: Monaco) {
    monaco.languages.register({ id: NAME });
    monaco.languages.setMonarchTokensProvider(NAME, {
        keywords: keywords,
        operators: operators,
        typeKeywords: typeKeywords,
        tokenizer: {
            root: tokenRoot,
            comment: [
                [/# .*$/, 'comment'],
            ],
        },
        ignoreCase: false
    });
    monaco.editor.defineTheme(THEME, {
        base: 'vs',
        inherit: true,
        rules: colorRules,
        colors: {}
    })
    monaco.languages.registerCompletionItemProvider(NAME, {
        triggerCharacters: ['G', 'g', 'p', 'P', 'M', 'm'],
        provideCompletionItems: function (model: any, position: any) {
            var word = model.getWordUntilPosition(position);
            var range = {
                startLineNumber: position.lineNumber,
                endLineNumber: position.lineNumber,
                startColumn: word.startColumn,
                endColumn: word.endColumn
            };
            return {
                suggestions: [
                    {
                        label: 'GET',
                        kind: monaco.languages.CompletionItemKind.Function,
                        insertText: 'GET',
                        detail: 'GET请求',
                        range: range
                    },
                    {
                        label: 'POST',
                        kind: monaco.languages.CompletionItemKind.Function,
                        insertText: 'POST',
                        detail: 'POST请求',
                        range: range
                    },
                    {
                        label: 'MOCK-REQ',
                        kind: monaco.languages.CompletionItemKind.Function,
                        insertText: 'MOCK-REQ:',
                        detail: 'mock请求',
                        range: range
                    },
                    {
                        label: 'MOCK-RES',
                        kind: monaco.languages.CompletionItemKind.Function,
                        insertText: 'MOCK-RES:',
                        detail: 'mock响应',
                        range: range
                    },
                ]
            };
        }
    })
}

/**
 * 添加命令
 * 
 * @param monaco 
 * @param editor 创建编辑器后获得的对象
 */
function addCommand(monaco: Monaco, editor: any) {
    const commandId = editor.addCommand(0, function () {
        console.log('run command');
    }, '')
    monaco.languages.registerCodeLensProvider(NAME, {
        provideCodeLenses: function (model: editor.ITextModel, token: CancellationToken): languages.ProviderResult<languages.CodeLensList> {
            const lines = model.getLinesContent();
            let count = 0;
            let start = 0;
            const range = [];
            for (let i = 0; i < lines.length; i++) {
                const ele = lines[i];
                // todo 应该判断是否有方法但是显示在###上
                if (ele.trim() && ele.startsWith('###')) {
                    count++;
                    if (count % 2 == 0) {
                        range.push(newRange(start, i))
                    }
                    start = i;
                }
            };
            const lenses = ranges2Lemses(commandId!, range);
            return {
                lenses: lenses,
                dispose: () => { }
            }
        }
    })

    const newRange = (start: number, end: number) => {
        return {
            startLineNumber: start + 1,
            startColumn: 0,
            endLineNumber: end + 1,
            endColumn: 3
        }
    }
    const ranges2Lemses = (id: string, range: any[]) => range.map(i => {
        return {
            range: i,
            id: 'run',
            command: {
                id: id,
                title: 'Run'
            }
        }
    });
}

