
var bob = {
  _name: "Bob",
  _friends: ["Sally", "Tom"],
  printFriends() {
    this._friends.forEach(f => console.log(this._name + " knows " + f));
  },
};