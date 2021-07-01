use std::thread;

pub struct ThreadPool{
    threads: Vec<thread::JoinHandle<()>>,
};

impl ThreadPool{
    ///新しいThreadPoolを生成する。
    /// 
    /// sizeがプールのスレッド数になる。
    /// 
    /// # パニック
    /// 
    /// sizeが0なら、`new`関数はパニックする。
    pub fn new(size: usize) -> ThreadPool{
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size{
            //スレッドを生成してベクタに格納する
        }

        ThreadPool{
            threads
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {

    }
}