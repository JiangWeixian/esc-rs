class Foo {
  constructor() {
    console.log(new.target);
  }
}

class Bar extends Foo {}

new Foo(); // => Foo
new Bar(); // => Bar