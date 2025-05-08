import * as fs from "node:fs";

const nativeNode = require("./index.node")

console.log(nativeNode)

const result = nativeNode.hello()
console.log(result)

nativeNode.printArgs(
    18,
    "jack",
    false,
    {name:"jack"},
    ["apple","banana"],
    ()=>{
        console.log("callback")
    }
)

console.log("returnString",nativeNode.returnString());
console.log("returnNumber",nativeNode.returnNumber());
console.log("returnBoolean",nativeNode.returnBoolean());
console.log("returnUndefined",nativeNode.returnUndefined());
console.log("returnNull",nativeNode.returnNull());
console.log("returnSimpleArr",nativeNode.returnSimpleArr());
console.log("returnArray",nativeNode.returnArray());
console.log("returnObject",nativeNode.returnObject());

const arrResult = nativeNode.returnArray();
for (let i = 0; i < arrResult.length; i++) {
    console.log(arrResult[i]);
}

const objResult = nativeNode.returnObject();
console.log("obj name is ",objResult["name"]);
for (let j = 0; j < objResult["user_hobby"].length; j++) {
    console.log("user ho",objResult["user_hobby"][j]);
}

const addFunction = nativeNode.returnFunction();
const addResult = addFunction(1,2)
console.log("addResult ",addResult)

nativeNode.readArray([1,2,3,"4","5","6"])

nativeNode.readObject({name:"jack",age:10})

nativeNode.readFunction((...args:any[])=>{
    args.forEach(item=>{
        console.log(item);
    })
})

// 读取图片为 Buffer
const imageBuffer = fs.readFileSync("./images.jpeg");

// 转换为 base64 字符串
const base64Image = imageBuffer.toString('base64');

// 如果你想加上 MIME 类型前缀（通常用于 HTML 或前端）
const dataUri = `data:image/jpeg;base64,${base64Image}`;

setInterval(()=>{
    console.time("printStart")
    nativeNode.print("render",dataUri)
    // nativeNode.print("main","good afternoon")
    console.timeEnd("printStart")
},3000)