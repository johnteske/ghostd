const elements = {}

//const getGroup = document.querySelector("#get");
//const setGroup = document.querySelector("#set");
//
//if (getGroup == null || setGroup == null) {
//  setMessage("element not found");
//  throw "element not found";
//}
//
//getGroup.addEventListener("keydown", onEnter(copyValue));
//setGroup.addEventListener("keydown", onEnter(setValue));
//
//const getButton = getGroup.querySelector("button");
//getButton != null && getButton.addEventListener("click", copyValue);
//
//const setButton = setGroup.querySelector("button");
//setButton != null && setButton.addEventListener("click", setValue);
//
//const img = document.querySelector("img");
//const message = document.querySelector("pre");
//const getInput = getGroup.querySelector("input");
//const setInput = setGroup.querySelector("input");
//
//if (img == null || message == null || getInput == null || setInput == null) {
//  setMessage("element not found");
//  throw "element not found";
//}

//

enum State {
  Start,
  Getting,
  Setting,
  Idle,
  Err,
  Final,
}

enum Action {
  StartOk,
  StartFail,
  GetOk,
  GetFail,
  SetOk,
  SetFail,
  IdleGet,
  IdleSet,
  Idle
}

let state = State.Start;

type Statemachine = {
  [key in State]: {
    fn: () => {};
    transitions: {
      [key in Action]: State
    };
  };
};

const stateMachine = {
  [State.Start]: {
    fn: () => {},
    transitions: {
      [Action.StartOk]: State.Getting,
      [Action.StartFail]: State.Final,
    },
  },
  [State.Getting]: {
    fn: () => {},
    transitions: {
      [Action.GetOk]: State.Idle,
      [Action.GetFail]: State.Err,
    },
  },
  [State.Idle]: {
    fn: () => {},
    transitions: {
      [Action.IdleGet]: State.Getting,
      [Action.IdleSet]: State.Setting,
    },
  },
  [State.Setting]: {
    fn: () => {},
    transitions: {
      [Action.SetOk]: State.Idle,
      [Action.SetFail]: State.Err,
    },
  },
  [State.Err]: {
    fn: () => {},
    transitions: {
      [Action.Idle]: State.Idle,
    },
  },
  [State.Idle]: {
    fn: () => {},
    transitions: {},
  },
};

//

//setMessage("loading...");
//
//fetch("/value")
//  .then((response) => response.json())
//  .then((data) => {
//    setGetInput(data.value);
//    setMessage("...");
//  })
//  .catch((error) => {
//    console.log(error);
//    setMessage("error getting value");
//    img.className = "swirl";
//  });
//
////
//
//function setMessage(str) {
//  message.innerHTML = str;
//}
//
//function selectAll(el) {
//  el.select();
//  el.setSelectionRange(0, el.value.length);
//}
//
//function onEnter(fn) {
//  return function (e) {
//    e.keyCode === 13 && fn();
//  };
//}
//
//function setGetInput(value) {
//  img.className = value === "" ? "" : "dance";
//  getInput.value = value;
//  selectAll(getInput);
//}
//
//function copyValue() {
//  navigator.clipboard.writeText(getInput.value).then(
//    () => {
//      setMessage("copied!");
//    },
//    () => {
//      setMessage("error copying value");
//      img.className = "swirl";
//    }
//  );
//}
//
//function setValue() {
//  fetch("/value", {
//    method: "POST",
//    headers: {
//      "Content-Type": "text",
//    },
//    body: setInput.value,
//  })
//    .then((response) => response.json())
//    .then((data) => {
//      setInput.value = "";
//      setGetInput(data.value);
//      setMessage("set new value!");
//    })
//    .catch((error) => {
//      console.log(error);
//      setMessage("error setting value");
//      img.className = "swirl";
//    });
//}

function stepState(action) {
  const nextState = stateMachine[state].transitions?.[action];
  if (nextState != null) {
    state = nextState;
  }
}
