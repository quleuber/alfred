// import comedy from "comedy";
const comedy = require("comedy");

const actor_system = comedy();

class MyActor {
  sayHello(to: string) {
    console.log(`Hello, ${to}!`);
  }
}

const actor_p = actor_system
  .rootActor()
  .then((root: any) => root.createChild(MyActor))
  .then((actor: any) => actor.send('sayHello', 42));

