mod flush;
mod write;
mod write_all;
mod write_vectored;

use flush::FlushFuture;
use write::WriteFuture;
use write_all::WriteAllFuture;
use write_vectored::WriteVectoredFuture;

use cfg_if::cfg_if;

use crate::io::IoSlice;
use crate::utils::extension_trait;

cfg_if! {
    if #[cfg(feature = "docs")] {
        use std::pin::Pin;
        use std::ops::{Deref, DerefMut};

        use crate::io;
        use crate::task::{Context, Poll};
    }
}

extension_trait! {
    #[doc = r#"
        Allows writing to a byte stream.

        This trait is a re-export of [`futures::io::AsyncWrite`] and is an async version of
        [`std::io::Write`].

        Methods other than [`poll_write`], [`poll_write_vectored`], [`poll_flush`], and
        [`poll_close`] do not really exist in the trait itself, but they become available when
        the prelude is imported:

        ```
        # #[allow(unused_imports)]
        use async_std::prelude::*;
        ```

        [`std::io::Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
        [`futures::io::AsyncWrite`]:
        https://docs.rs/futures-preview/0.3.0-alpha.17/futures/io/trait.AsyncWrite.html
        [`poll_write`]: #tymethod.poll_write
        [`poll_write_vectored`]: #method.poll_write_vectored
        [`poll_flush`]: #tymethod.poll_flush
        [`poll_close`]: #tymethod.poll_close
    "#]
    pub trait Write [WriteExt: futures_io::AsyncWrite] {
        #[doc = r#"
            Attempt to write bytes from `buf` into the object.
        "#]
        fn poll_write(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<io::Result<usize>>;

        #[doc = r#"
            Attempt to write bytes from `bufs` into the object using vectored IO operations.
        "#]
        fn poll_write_vectored(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            bufs: &[IoSlice<'_>]
        ) -> Poll<io::Result<usize>> {
            unreachable!("this impl only appears in the rendered docs")
        }

        #[doc = r#"
            Attempt to flush the object, ensuring that any buffered data reach
            their destination.
        "#]
        fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>>;

        #[doc = r#"
            Attempt to close the object.
        "#]
        fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>>;

        #[doc = r#"
            Writes some bytes into the byte stream.

            Returns the number of bytes written from the start of the buffer.

            If the return value is `Ok(n)` then it must be guaranteed that
            `0 <= n <= buf.len()`. A return value of `0` typically means that the underlying
            object is no longer able to accept bytes and will likely not be able to in the
            future as well, or that the buffer provided is empty.

            # Examples

            ```no_run
            # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
            #
            use async_std::fs::File;
            use async_std::prelude::*;

            let mut file = File::create("a.txt").await?;

            let n = file.write(b"hello world").await?;
            #
            # Ok(()) }) }
            ```
        "#]
        fn write<'a>(
            &'a mut self,
            buf: &'a [u8],
        ) -> impl Future<Output = io::Result<usize>> + 'a [WriteFuture<'a, Self>]
        where
            Self: Unpin,
        {
            WriteFuture { writer: self, buf }
        }

        #[doc = r#"
            Flushes the stream to ensure that all buffered contents reach their destination.

            # Examples

            ```no_run
            # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
            #
            use async_std::fs::File;
            use async_std::prelude::*;

            let mut file = File::create("a.txt").await?;

            file.write_all(b"hello world").await?;
            file.flush().await?;
            #
            # Ok(()) }) }
            ```
        "#]
        fn flush(&mut self) -> impl Future<Output = io::Result<()>> + '_ [FlushFuture<'_, Self>]
        where
            Self: Unpin,
        {
            FlushFuture { writer: self }
        }

        #[doc = r#"
            Like [`write`], except that it writes from a slice of buffers.

            Data is copied from each buffer in order, with the final buffer read from possibly
            being only partially consumed. This method must behave as a call to [`write`] with
            the buffers concatenated would.

            The default implementation calls [`write`] with either the first nonempty buffer
            provided, or an empty one if none exists.

            [`write`]: #tymethod.write
        "#]
        fn write_vectored<'a>(
            &'a mut self,
            bufs: &'a [IoSlice<'a>],
        ) -> impl Future<Output = io::Result<usize>> + 'a [WriteVectoredFuture<'a, Self>]
        where
            Self: Unpin,
        {
            WriteVectoredFuture { writer: self, bufs }
        }

        #[doc = r#"
            Writes an entire buffer into the byte stream.

            This method will continuously call [`write`] until there is no more data to be
            written or an error is returned. This method will not return until the entire
            buffer has been successfully written or such an error occurs.

            [`write`]: #tymethod.write

            # Examples

            ```no_run
            # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
            #
            use async_std::fs::File;
            use async_std::prelude::*;

            let mut file = File::create("a.txt").await?;

            file.write_all(b"hello world").await?;
            #
            # Ok(()) }) }
            ```

            [`write`]: #tymethod.write
        "#]
        fn write_all<'a>(
            &'a mut self,
            buf: &'a [u8],
        ) -> impl Future<Output = io::Result<()>> + 'a [WriteAllFuture<'a, Self>]
        where
            Self: Unpin,
        {
            WriteAllFuture { writer: self, buf }
        }
    }

    impl<T: Write + Unpin + ?Sized> Write for Box<T> {
        fn poll_write(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<io::Result<usize>> {
            unreachable!("this impl only appears in the rendered docs")
        }

        fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
            unreachable!("this impl only appears in the rendered docs")
        }

        fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
            unreachable!("this impl only appears in the rendered docs")
        }
    }

    impl<T: Write + Unpin + ?Sized> Write for &mut T {
        fn poll_write(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<io::Result<usize>> {
            unreachable!("this impl only appears in the rendered docs")
        }

        fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
            unreachable!("this impl only appears in the rendered docs")
        }

        fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
            unreachable!("this impl only appears in the rendered docs")
        }
    }

    impl<P> Write for Pin<P>
    where
        P: DerefMut + Unpin,
        <P as Deref>::Target: Write,
    {
        fn poll_write(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<io::Result<usize>> {
            unreachable!("this impl only appears in the rendered docs")
        }

        fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
            unreachable!("this impl only appears in the rendered docs")
        }

        fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
            unreachable!("this impl only appears in the rendered docs")
        }
    }

    impl Write for Vec<u8> {
        fn poll_write(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<io::Result<usize>> {
            unreachable!("this impl only appears in the rendered docs")
        }

        fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
            unreachable!("this impl only appears in the rendered docs")
        }

        fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
            unreachable!("this impl only appears in the rendered docs")
        }
    }
}
