use std::thread;
use std::cmp::Ordering;
use std::sync::{mpsc,Arc,Mutex};

#[derive(Debug)]
pub enum PoolCreationErr{
	InvalidInit,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message{
	Job(Job),
	Shutdown,
}

pub struct ThreadPool {
	workers: Vec<Worker>,
	sender: mpsc::Sender<Message>,
}

impl ThreadPool{
	pub fn new(size: usize) -> Result<ThreadPool,PoolCreationErr> {
		if let Ordering::Less = size.cmp(&1){
			return Err(PoolCreationErr::InvalidInit);
		}

		let mut threads = Vec::with_capacity(size);
		let (sender, receiver) = mpsc::channel();

		let recivref = Arc::new(Mutex::new(receiver));
		for i in 0..size {
			threads.push(Worker::new(i,Arc::clone(&recivref)));
		}

		Ok(ThreadPool{workers: threads, sender: sender})
	}
	pub fn execute(&self, func: impl FnOnce() + Send + 'static){
		let job = Box::new(func);

		self.sender.send(Message::Job(job)).unwrap();
	}
}

impl Drop for ThreadPool{
	fn drop(&mut self){
		for _ in &mut self.workers{
			self.sender.send(Message::Shutdown).unwrap()
		}


		for worker in &mut self.workers {
			println!("Shutting down worker {}.", worker.id);

			if let Some(thread) = worker.handle.take(){
				thread.join().unwrap();
			}
		}
	}
}


struct Worker{
	id: usize,
	handle: Option<thread::JoinHandle<()>>,
}

impl Worker{
	fn new(id: usize, reciver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker{
		let thread = thread::spawn(move||{
			loop{
				let message = reciver.lock().unwrap().recv().unwrap();

				match message{
					Message::Job(job) => {
						println!("Worker {} got a job.", id);

						job();

						println!("Worker {} is done!", id);
					}
					Message::Shutdown => {
						break;
					}
				}

			}
		});
		println!("Thread {} created!", id);
		Worker{id: id, handle: Some(thread)}
	}
}


#[cfg(test)]
mod tests{
	use super::*;

	#[test]
	fn tp_err(){
		assert!(matches!(ThreadPool::new(0), Err(PoolCreationErr::InvalidInit)));
	}
}