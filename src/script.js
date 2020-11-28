const getInput = document.querySelector("#get input");
const setInput = document.querySelector("#set input");

const selectAll = (el) => {
  el.select();
  el.setSelectionRange(0, el.value.length);
};

document.querySelector("#get button").addEventListener("click", () => {
  selectAll(getInput);
  document.execCommand("copy");
});

document.querySelector("#set button").addEventListener("click", () => {
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
});

fetch("/value")
  .then((response) => response.text())
  .then((data) => {
    getInput.value = data;
    selectAll(getInput);
  });
