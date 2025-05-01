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
    console.log("user hobby ",objResult["user_hobby"][j]);
}

const addFunction = nativeNode.returnFunction();
const addResult = addFunction(1,2)
console.log("addResult ",addResult)