// State

let state = State.Start;

const elements: { 
  img: Element | null
  message: Element | null
  getGroup: Element | null
  getInput: HTMLInputElement | null
  getButton: HTMLInputElement | null
  setGroup: Element | null
  setInput: HTMLInputElement | null
  setButton: HTMLInputElement | null
 } = {
  img: document.querySelector("img"),
  message: document.querySelector("pre"),
  getGroup: document.querySelector("#get"),
  getInput : document.querySelector("#get input"),
  getButton : document.querySelector("#get button"),
  setGroup: document.querySelector("#set"),
  setInput : document.querySelector("#set input"),
  setButton : document.querySelector("#set button"),
};

type AnimationClassName = "dance" | "swirl" | "";

// State Machine

const enum Action {
  OK,
  FAIL,
  GET,
  SET,
}

const enum State {
  Start,
  Getting,
  Idle,
  Setting,
  Err,
  Final,
}

const states: {
  [key in State]: {
    fn?: () => void;
    transitions?: { [key in Action]?: State };
    message: string;
    animation?: "dance" | "swirl";
  };
} = {
  [State.Start]: {
    fn: start,
    transitions: { [Action.OK]: State.Getting, [Action.FAIL]: State.Final },
    message: "loading..."
  },
  [State.Getting]: {
    fn: getValue,
    transitions: { [Action.OK]: State.Idle, [Action.FAIL]: State.Err },
    message: "getting..."
  },
  [State.Idle]: {
    transitions: { [Action.GET]: State.Getting, [Action.SET]: State.Setting },
    message: "..."
  },
  [State.Setting]: {
    fn: postValue,
    transitions: { [Action.OK]: State.Getting, [Action.FAIL]: State.Err },
    message: "setting..."
  },
  [State.Err]: {
    transitions: { [Action.OK]: State.Idle },
    animation: "swirl",
    message: "uh oh..."
  },
  [State.Final]: {
    message: "something went terrible wrong..."
  },
};

function transition(action: Action) {
  const nextState = states[state].transitions?.[action];
  if (nextState != null) {
    state = nextState;
    states[state].fn?.();
  }

  // button state
  elements.getButton!.disabled = state !== State.Idle
  elements.setButton!.disabled = state !== State.Idle

  setMessage(states[state].message);
  setAnimation(states[state].animation);
}

//

function start() {
  // if any of the elements are null, fail
  if (!Object.keys(elements).every(Boolean)) {
    setMessage("failed to find some elements");
    return transition(Action.FAIL)
  }

  elements.getGroup!.addEventListener("keydown", onEnter(copyValue) as EventListener); // KeyboardEvent is not inferred from "keydown" type
  elements.getGroup!.querySelector("button")?.addEventListener("click", copyValue);

  const setValue = () => { transition(Action.SET) } 
  elements.setGroup!.addEventListener("keydown", onEnter(setValue) as EventListener); // KeyboardEvent is not inferred from "keydown" type
  elements.setGroup!.querySelector("button")?.addEventListener("click", setValue);

    return transition(Action.OK)
}

async function getValue() {
  await fetch("/value")
  .then((response) => response.json())
  .then((data) => {
    console.log(data);

    const getInput = elements.getInput!
    getInput.value = data.value;
    getInput.focus();
    getInput.select();
    getInput.setSelectionRange(0, data.value.length);
    
    transition(Action.OK); 
  })
  .catch((error) => {
    console.log(error);
    setMessage("error getting value");
    transition(Action.FAIL); 
  });

}

function setAnimation(animation: AnimationClassName = "") {
    elements.img!.className = animation;
}

function setMessage(str: string) {
  console.log(str);
  elements.message!.innerHTML = str;
}

function onEnter(fn: () => void)  {
  return function (e: KeyboardEvent) {
    e.keyCode === 13 && fn();
  };
}

//

function copyValue() {
  navigator.clipboard.writeText(elements.getInput!.value).then(
    () => {
      setMessage("copied!");
    },
    () => {
      setMessage("error copying value");
      // TODO transition to State.Err?
    }
  );
}

async function postValue() {
  fetch("/value", {
    method: "POST",
    headers: {
      "Content-Type": "text",
    },
    body: elements.setInput!.value,
  })
    .then((response) => response.json())
    .then((data) => {
      elements.setInput!.value = "";
      setMessage("set new value!");
      transition(Action.OK)
    })
    .catch((error) => {
      console.log(error);
      setMessage("error setting value");
      transition(Action.FAIL)
    });
}

// start state machine
// TODO this doesn't include any of the UI updates from 'transition'
states[state].fn?.()
