try {
  throw 0;
} catch {
  doSomethingWhichDoesNotCareAboutTheValueThrown();
}