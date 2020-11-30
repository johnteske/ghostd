const img = document.querySelector("img");
const message = document.querySelector("pre");
const getInput = document.querySelector("#get input");
const setInput = document.querySelector("#set input");

document.querySelector("#get").addEventListener("keydown", onEnter(copyValue));
document.querySelector("#get button").addEventListener("click", copyValue);

document.querySelector("#set").addEventListener("keydown", onEnter(setValue));
document.querySelector("#set button").addEventListener("click", setValue);

setMessage("loading...");

fetch("/value")
  .then((response) => response.json())
  .then((data) => {
    setGetInput(data.value);
    setMessage("...");
  })
  .catch((error) => {
    console.log(error);
    setMessage("error getting value");
    img.className = "swirl";
  });

//

function setMessage(str) {
  message.innerHTML = str;
}

function selectAll(el) {
  el.select();
  el.setSelectionRange(0, el.value.length);
}

function onEnter(fn) {
  return function (e) {
    e.keyCode === 13 && fn();
  };
}

function setGetInput(value) {
  img.className = ["", "UNSET"].includes(value) ? "" : "dance";
  getInput.value = value;
  selectAll(getInput);
}

function copyValue() {
  selectAll(getInput);
  document.execCommand("copy");
  setMessage("copied!");
}

function setValue() {
  fetch("/value", {
    method: "POST",
    header: {
      "Content-Type": "text",
    },
    body: setInput.value,
  })
    .then((response) => response.json())
    .then((data) => {
      setInput.value = null;
      setGetInput(data.value);
      setMessage("set new value!");
    })
    .catch((error) => {
      console.log(error);
      setMessage("error setting value");
      img.className = "swirl";
    });
}
