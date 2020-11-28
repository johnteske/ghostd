const input = document.querySelector("input");

const selectAll = () => {
  input.select();
  input.setSelectionRange(0, input.value.length);
};

document.querySelector("button").addEventListener("click", () => {
  selectAll();
  document.execCommand("copy");
});

fetch("/value")
  .then((response) => response.text())
  .then((data) => {
    input.value = data;
    selectAll();
  });
