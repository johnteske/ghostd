const getInput = document.querySelector("#get input");

const selectAll = (el) => {
  el.select();
  el.setSelectionRange(0, el.value.length);
};

document.querySelector("#get button").addEventListener("click", () => {
  selectAll(getInput);
  document.execCommand("copy");
});

fetch("/value")
  .then((response) => response.text())
  .then((data) => {
    getInput.value = data;
    selectAll(getInput);
  });
