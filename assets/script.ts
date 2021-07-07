let messages = "";
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
  ERR,
  GET,
  SET,
}

const enum State {
  Initial,
  Start,
  Getting,
  Idle,
  Setting,
  Err,
  Fatal,
}

const states: {
  [key in State]: {
    fn?: () => void;
    transitions?: { [key in Action]?: State };
    message?: string;
    animation?: AnimationClassName;
  };
} = {
  [State.Initial]: {
    transitions: { [Action.OK]: State.Start },
  },
  [State.Start]: {
    fn: start,
    transitions: { [Action.OK]: State.Getting, [Action.ERR]: State.Fatal },
    message: "loading...",
  },
  [State.Getting]: {
    fn: getValue,
    transitions: { [Action.OK]: State.Idle, [Action.ERR]: State.Err },
  },
  [State.Idle]: {
    transitions: { [Action.GET]: State.Getting, [Action.SET]: State.Setting },
  },
  [State.Setting]: {
    fn: postValue,
    transitions: { [Action.OK]: State.Getting, [Action.ERR]: State.Err },
    message: "setting...",
  },
  [State.Err]: {
    transitions: { [Action.GET]: State.Getting, [Action.SET]: State.Setting },
    animation: "swirl",
  },
  [State.Fatal]: {
    message: "something went terribly wrong\n",
    animation: "swirl",
  },
};

function transition(action: Action) {
  const nextState = states[state].transitions?.[action];

  if (nextState == null) {
    appendMessage("not a valid transition!\n");
    return;
  }

  state = nextState;
  states[state].fn?.();

  const msg = states[state].message;
  if (msg != null) {
    appendMessage(msg);
  }

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
    appendMessage("failed to find some elements\n");
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

  appendMessage("ok\n");
  return transition(Action.OK);
}

async function getValue() {
  await fetch("/value")
    .then((response: Response) => response.text())
    .then((data) => {
      const getInput = elements.getInput!;
      getInput.value = data;
      getInput.focus();
      getInput.select();
      getInput.setSelectionRange(0, data.length);
      transition(Action.OK);
    })
    .catch((error) => {
      appendMessage("error getting value!\n");
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
    .then((response: Response) => {
      elements.setInput!.value = "";
      switch (response.status) {
        case 204:
          appendMessage("ok\n");
          transition(Action.OK);
          break;
        case 413:
          appendMessage("too large!\n");
          transition(Action.ERR);
        default:
          break;
      }
    })
    .catch((error) => {
      appendMessage("error");
      console.log(error);
      transition(Action.FAIL);
    });
}

// UI
function appendMessage(str: string) {
  messages += str;
  messages = messages.split("\n").slice(-3).join("\n");
  console.log(str);
  elements.message!.innerHTML = messages;
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
      appendMessage("copied!\n");
    },
    () => {
      appendMessage("error copying value\n");
    }
  );
}

// helpers

function onEnter(fn: () => void) {
  return function (e: KeyboardEvent) {
    e.keyCode === 13 && fn();
  };
}
