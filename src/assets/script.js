const getGroup = document.querySelector("#get");
const setGroup = document.querySelector("#set");

if (getGroup == null || setGroup == null) {
  setMessage("element not found");
  throw "element not found";
}

setGroup.addEventListener("keydown", onEnter(setValue));

const getButton = getGroup.querySelector("button");
getButton != null && getButton.addEventListener("click", copyValue);
getGroup.addEventListener("keydown", onEnter(copyValue));

const setButton = setGroup.querySelector("button");
setButton != null && setButton.addEventListener("click", setValue);

const img = document.querySelector("img");
const message = document.querySelector("pre");
const getInput = getGroup.querySelector("input");
const setInput = setGroup.querySelector("input");

if (img == null || message == null || getInput == null || setInput == null) {
  setMessage("element not found");
  throw "element not found";
}

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

navigator.permissions.query({ name: "clipboard-write" }).then((p) => {
  const getButton = getGroup.querySelector("button");
  if (p.state === "denied") {
    return;
  }

  if (getButton != null) {
    getButton.disabled = false;
    getButton.addEventListener("click", copyValue);
  }

  getGroup.addEventListener("keydown", onEnter(copyValue));
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
  img.className = value === "" ? "" : "dance";
  getInput.value = value;
  selectAll(getInput);
}

function copyValue() {
  navigator.clipboard.writeText(getInput.value).then(
    () => {
      setMessage("copied!");
    },
    () => {
      setMessage("error copying value");
      img.className = "swirl";
    }
  );
}

function setValue() {
  fetch("/value", {
    method: "POST",
    headers: {
      "Content-Type": "text",
    },
    body: setInput.value,
  })
    .then((response) => response.json())
    .then((data) => {
      setInput.value = "";
      setGetInput(data.value);
      setMessage("set new value!");
    })
    .catch((error) => {
      console.log(error);
      setMessage("error setting value");
      img.className = "swirl";
    });
}
