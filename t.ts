import { DirectoryWatcher } from ".";

const d = DirectoryWatcher.new((_, event) => {
  console.log(JSON.parse(event));
});

d.watch("C:\\");

setInterval(() => {
  d.getWatchedPaths();
}, 1000);
