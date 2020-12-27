const elements: {
  img: Element | null;
  message: Element | null;
  getGroup: Element | null;
  getInput: HTMLInputElement | null;
  getButton: HTMLInputElement | null;
  setGroup: Element | null;
  setInput: HTMLInputElement | null;
  setButton: HTMLInputElement | null;
} = {
  img: document.querySelector("img"),
  message: document.querySelector("pre"),
  getGroup: document.querySelector("#get"),
  getInput: document.querySelector("#get input"),
  getButton: document.querySelector("#get button"),
  setGroup: document.querySelector("#set"),
  setInput: document.querySelector("#set input"),
  setButton: document.querySelector("#set button"),
};

// State Machine

const enum Action {
  OK,
  FAIL,
  GET,
  SET,
}

const enum State {
  Initial,
  Start,
  Getting,
  Idle,
  Setting,
  Final,
}

const states: {
  [key in State]: {
    fn?: () => void;
    transitions?: { [key in Action]?: State };
    message: string;
    animation?: AnimationClassName;
  };
} = {
  [State.Initial]: {
    transitions: { [Action.OK]: State.Start },
    message: "",
  },
  [State.Start]: {
    fn: start,
    transitions: { [Action.OK]: State.Getting, [Action.FAIL]: State.Final },
    message: "loading...",
  },
  [State.Getting]: {
    fn: getValue,
    transitions: { [Action.OK]: State.Idle, [Action.FAIL]: State.Final },
    message: "getting...",
  },
  [State.Idle]: {
    transitions: { [Action.GET]: State.Getting, [Action.SET]: State.Setting },
    message: "...",
  },
  [State.Setting]: {
    fn: postValue,
    transitions: { [Action.OK]: State.Getting, [Action.FAIL]: State.Final },
    message: "setting...",
  },
  [State.Final]: {
    animation: "swirl",
    message: "uh oh... something went terribly wrong",
  },
};

function transition(action: Action) {
  const nextState = states[state].transitions?.[action];

  if (nextState == null) {
    setMessage("not a valige transition!");
    return;
  }

  state = nextState;
  states[state].fn?.();

  setMessage(states[state].message);
  setAnimation(states[state].animation);
  setButtonState(state);
}

let state = State.Initial;
transition(Action.OK);

//

function start() {
  // if any of the elements are null, fail
  if (!Object.keys(elements).every(Boolean)) {
    setMessage("failed to find some elements");
    return transition(Action.FAIL);
  }

  elements.getGroup!.addEventListener(
    "keydown",
    onEnter(copyValue) as EventListener
  ); // KeyboardEvent is not inferred from "keydown" type
  elements
    .getGroup!.querySelector("button")
    ?.addEventListener("click", copyValue);

  const setValue = () => {
    transition(Action.SET);
  };
  elements.setGroup!.addEventListener(
    "keydown",
    onEnter(setValue) as EventListener
  ); // KeyboardEvent is not inferred from "keydown" type
  elements
    .setGroup!.querySelector("button")
    ?.addEventListener("click", setValue);

  return transition(Action.OK);
}

async function getValue() {
  await fetch("/value")
    .then(toJson)
    .then((data) => {
      const getInput = elements.getInput!;
      getInput.value = data.value;
      getInput.focus();
      getInput.select();
      getInput.setSelectionRange(0, data.value.length);
      transition(Action.OK);
    })
    .catch((error) => {
      setMessage("error getting value");
      console.log(error);
      transition(Action.FAIL);
    });
}

async function postValue() {
  fetch("/value", {
    method: "POST",
    headers: {
      "Content-Type": "text",
    },
    body: elements.setInput!.value,
  })
    .then(toJson)
    .then((data) => {
      elements.setInput!.value = "";
      setMessage("set new value!");
      transition(Action.OK);
    })
    .catch((error) => {
      setMessage("error setting value");
      console.log(error);
      transition(Action.FAIL);
    });
}

// UI

function setMessage(str: string) {
  console.log(str);
  elements.message!.innerHTML = str;
}

type AnimationClassName = "dance" | "swirl" | "";

function setAnimation(animation: AnimationClassName = "") {
  elements.img!.className = animation;
}

function setButtonState(state: State) {
  elements.getButton!.disabled = state !== State.Idle;
  elements.setButton!.disabled = state !== State.Idle;
}

function copyValue() {
  navigator.clipboard.writeText(elements.getInput!.value).then(
    () => {
      setMessage("copied!");
    },
    () => {
      setMessage("error copying value");
    }
  );
}

// helpers

function onEnter(fn: () => void) {
  return function (e: KeyboardEvent) {
    e.keyCode === 13 && fn();
  };
}

function toJson(response: Response) {
  return response.json();
}
