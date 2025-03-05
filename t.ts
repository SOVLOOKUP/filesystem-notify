import { DirectoryWatcher } from ".";

const d = DirectoryWatcher.new((_, event) => {
  console.log(JSON.parse(event));
});

d.watch("C:\\");

setInterval(() => {
  console.log(d.getWatchedPaths());
}, 1000);
