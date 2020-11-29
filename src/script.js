const getInput = document.querySelector("#get input");
const setInput = document.querySelector("#set input");

document.querySelector("#get").addEventListener("keydown", onEnter(copyValue));
document.querySelector("#get button").addEventListener("click", copyValue);

document.querySelector("#set").addEventListener("keydown", onEnter(setValue));
document.querySelector("#set button").addEventListener("click", setValue);

fetch("/value")
  .then((response) => response.text())
  .then((data) => {
    getInput.value = data;
    selectAll(getInput);
  });

//

function selectAll(el) {
  el.select();
  el.setSelectionRange(0, el.value.length);
}

function onEnter(fn) {
  return function (e) {
    e.keyCode === 13 && fn();
  };
}

function copyValue() {
  selectAll(getInput);
  document.execCommand("copy");
}

function setValue() {
  fetch("/value", {
    method: "POST",
    header: {
      "Content-Type": "text",
    },
    body: setInput.value,
  })
    .then((response) => response.text())
    .then((data) => {
      setInput.value = null;
      getInput.value = data;
      selectAll(getInput);
    })
    .catch((error) => {
      console.error(error);
    });
}
