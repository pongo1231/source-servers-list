use futures_util::{Sink, SinkExt, Stream};
use pin_project::pin_project;
use serde::Serialize;
use std::{
	pin::Pin,
	task::{Context, Poll},
};

#[pin_project]
pub struct WSStream<T> {
	#[pin]
	pub inner: T,
}

impl<T> WSStream<T>
where
	T: Unpin,
{
	pub async fn send<M, S>(&mut self, msg: M) -> Result<(), T::Error>
	where
		T: Sink<S>,
		M: Serialize,
		S: From<String>,
	{
		self.inner
			.send(serde_json::to_string(&msg).unwrap().into())
			.await
	}
}

impl<T> Stream for WSStream<T>
where
	T: Stream + Unpin,
{
	type Item = T::Item;

	fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
		self.project().inner.as_mut().poll_next(cx)
	}
}
