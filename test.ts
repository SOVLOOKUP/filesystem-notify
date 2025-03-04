import { DirectoryWatcher } from ".";

const d = DirectoryWatcher.new((_, event) => {
  console.log(111, JSON.parse(event));
});

d.watch("C:\\Users\\SOVLOOKUP\\Desktop\\notify-ts");

setInterval(() => {
  d.getWatchedPaths();
}, 1000);
