let canCopy = false;

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
    message: "something went terribly wrong",
    animation: "swirl",
  },
};

function transition(action: Action) {
  const nextState = states[state].transitions?.[action];

  if (nextState == null) {
    setMessage("not a valid transition!");
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

// initialize UI event listeners
async function start() {
  // if any of the elements are null, fail
  if (!Object.keys(elements).every(Boolean)) {
    setMessage("failed to find some elements");
    return transition(Action.FAIL);
  }

  // ts complains that clipboard-write cannot be assigned to PermissionName but the spec says otherwise:
  // https://w3c.github.io/permissions/#enumdef-permissionname
  await navigator.permissions
    .query({ name: "clipboard-write" as PermissionName })
    .then((p) => {
      if (p.state === "denied") {
        elements.getButton!.disabled = true;
        return;
      }

      canCopy = true;

      elements.getGroup!.addEventListener(
        "keydown",
        onEnter(copyValue) as EventListener
      ); // KeyboardEvent is not inferred from "keydown" type
      elements
        .getGroup!.querySelector("button")
        ?.addEventListener("click", copyValue);
    });

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
    .then(toText)
    .then((data) => {
      const getInput = elements.getInput!;
      getInput.value = data;
      getInput.focus();
      getInput.select();
      getInput.setSelectionRange(0, data.length);
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
      "Content-Type": "text/plain",
    },
    body: elements.setInput!.value,
  })
    .then(toText)
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

type AnimationClassName = "swirl" | "";

function setAnimation(animation: AnimationClassName = "") {
  elements.img!.className = animation;
}

function setButtonState(state: State) {
  if (canCopy) {
    elements.getButton!.disabled = state !== State.Idle;
  }
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

function toText(response: Response) {
  if (response.ok) {
    return response.text();
  }

  throw new Error(`${response.status}`)
}
