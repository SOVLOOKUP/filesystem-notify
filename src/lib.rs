use napi::{
  threadsafe_function::{ErrorStrategy, ThreadsafeFunction},
  JsFunction,
};
use napi_derive::napi;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use serde::Serialize;
use std::{path::Path, thread::spawn};
use tokio::sync::mpsc::channel;

// 定义文件系统事件结构
#[derive(Debug, Serialize)]
pub struct NotifyEvent {
  pub event: Event,
  pub timestamp: f64,
}

// 定义文件监听器结构体
#[napi]
pub struct DirectoryWatcher {
  watcher: RecommendedWatcher,
  paths: std::collections::HashSet<String>,
}

#[napi]
impl DirectoryWatcher {
  // 创建新的监听器实例
  #[napi(ts_args_type = "callback: (err: null | Error, result: string) => void | Promise<void>")]
  pub fn new(callback: JsFunction) -> Self {
    let (tx, mut rx) = channel(32);

    // 创建线程安全的回调函数
    let tsfn: ThreadsafeFunction<String, ErrorStrategy::CalleeHandled> = callback
      .create_threadsafe_function(
        32,
        |ctx: napi::threadsafe_function::ThreadSafeCallContext<String>| {
          ctx.env.create_string(&ctx.value).map(|v| vec![v])
        },
      )
      .unwrap();

    let watcher = RecommendedWatcher::new(
      move |res: Result<Event, notify::Error>| {
        if let Ok(event) = res {
          let notify_event = convert_event(event);
          if let Ok(json) = serde_json::to_string(&notify_event) {
            let _ = tx.blocking_send(json.clone());
          }
        }
      },
      Config::default(),
    )
    .unwrap();

    spawn(move || {
      while let Some(value) = rx.blocking_recv() {
        let tsfn_clone = tsfn.clone();
        tsfn_clone.call(
          Ok(value),
          napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
        );
      }
    });

    DirectoryWatcher {
      watcher,
      paths: std::collections::HashSet::new(),
    }
  }

  // 开始监听指定目录
  #[napi]
  pub fn watch(&mut self, path: String) -> napi::Result<()> {
    self
      .watcher
      .watch(Path::new(path.as_str()), RecursiveMode::Recursive)
      .unwrap();
    self.paths.insert(path.to_string());
    Ok(())
  }

  // 取消监听指定目录
  #[napi]
  pub fn unwatch(&mut self, path: String) -> napi::Result<()> {
    self.watcher.unwatch(Path::new(path.as_str())).unwrap();
    self.paths.remove(path.as_str());
    Ok(())
  }

  // 获取当前所有被监听的目录
  #[napi]
  pub fn get_watched_paths(&self) -> Vec<String> {
    self.paths.iter().cloned().collect()
  }
}

// 辅助函数：转换 notify 事件到我们的事件类型
fn convert_event(event: Event) -> NotifyEvent {
  let timestamp = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap()
    .as_secs_f64();

  NotifyEvent { event, timestamp }
}
