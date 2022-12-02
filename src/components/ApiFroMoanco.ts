import { Monaco as M } from "@monaco-editor/loader";
import { editor, CancellationToken, languages } from 'monaco-editor';
type Monaco = M;

const NAME = 'api';
const THEME = 'apiTheme';

const keywords = ['GET', 'POST', 'MOCK', 'OK', 'ERR', 'MUST', 'SET'];
const operators = ['=>', '=', '+', '|', '###'];
const typeKeywords = [
    'str', 'bool', 'num', 'array',
    'u8', 'u16', 'u32', 'u64',
    'i8', 'i16', 'i32', 'i64',
    'f16', 'f32'];

//todo类型关键字和操作符颜色
// [正则, token, color]
const colorDefine: [RegExp, string, string][] = [
    [/###/, 'split', '#d16951'], // 分隔符: 固定###
    [/#.*$/, 'comment', '#438a55'], // 注释: 以#开头直到结尾
    [/@([\w]*)/, 'define', '#3ac1f1'], // 定义变量: @开头直到非字符
    [/\b(GET|POST|MOCK|OK|ERR|MUST|SET)\b/, 'keyword', '#004d99'], // 方法关键字
    [/"[^"]*"/, 'string', '#ce723b'], // 字符串: 以"开头以"结尾中间没有"的部分
    [/\b(str|bool|num|u8|u16|u32|u64|i8|i16|i32|i64|f16|f32|array)\b/, 'typekey', '#3ac9b0'], // todo 使用typeKeywords去匹配
    // [/({|}|[|]|<|>|\(|\))/, 'operator', '#ffff00'], // todo 使用括号去匹配
    // [/\/[^\s]*/, 'url', '#ff3d00'],
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
    });
    monaco.languages.registerCompletionItemProvider(NAME, {
        triggerCharacters: ['G', 'g', 'p', 'P', 'M', 'm', '#'],
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
                        label: 'MOCK',
                        kind: monaco.languages.CompletionItemKind.Function,
                        insertText: 'MOCK',
                        detail: 'mock请求和响应',
                        range: range
                    },
                    {
                        label: 'request',
                        kind: monaco.languages.CompletionItemKind.Keyword,
                        insertText: [
                            '##',
                            '${0}',
                            '###'
                        ].join('\n'),
                        insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
                        detail: 'mock请求和响应',
                        range: range
                    },
                ]
            };
        }
    });
    monaco.languages.setLanguageConfiguration(NAME, {
        brackets: [
            ['{', '}'],
            ['(', ')'],
            ['<', '>'],
        ],
        autoClosingPairs: [
            { open: '{', close: '}' },
            { open: '(', close: ')' },
            { open: '<', close: '>' },
            { open: '###', close: '###', notIn: ['string', 'comment'] },
        ]
    });
    monaco.languages.registerHoverProvider(NAME, {
        provideHover: function (model, position, token) {
            const w = model.getWordAtPosition(position);
            const line = model.getLineContent(position.lineNumber);
            if (w && line) {
                const word = w.word;
                // todo 需要判断是否在 @变量 的位置内, 且是在使用时
                if (line.includes(`@${word}`)) {
                    return {
                        contents: [
                            { value: `${word}` },
                            { value: '显示变量值' }
                        ]
                    }
                }
            }
        },
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
                title: 'RUN'
            }
        }
    });
}

