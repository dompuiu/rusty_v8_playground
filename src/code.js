// let done = false;

// const x = async () => {
//   const r = await Promise.resolve(5);
//   return r;
// };
// (async () => {
//   const r = await x();
//   done = true;
// })();

// const a = (async function () {
//   var body = await Promise.resolve(5);
//   return 6;
// })();

// a;

// async function AsyncCall() {
//   let promise = Promise.resolve(1);
//   let result = await promise;

//   return result;
// }
// await AsyncCall();

console = console2;

setTimeout(() => {
  console.log("from inside timeout");
}, 3000);

console.log("aaa");

new Promise((resolve) => {
  const start = Date.now();

  while (true) {
    const end = Date.now();
    if (end - start === 2000) {
      break;
    }
  }

  console.log("aaa");
  // console.log(false);
  resolve(100);
});
