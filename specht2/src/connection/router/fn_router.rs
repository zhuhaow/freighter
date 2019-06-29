// MIT License

// Copyright (c) 2019 Zhuhao Wang

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use super::Router;
use freighter::core::{Endpoint, Error};
use tokio::prelude::*;

pub struct FnRouter<'a, P, Fut>
where
    P: AsyncRead + AsyncWrite + Send,
    Fut: Future<Item = P, Error = Error>,
{
    inner: Box<FnMut(&Endpoint) -> Fut + Send + 'a>,
}

impl<'a, P, Fut> FnRouter<'a, P, Fut>
where
    P: AsyncRead + AsyncWrite + Send,
    Fut: Future<Item = P, Error = Error>,
{
    pub fn new<F: FnMut(&Endpoint) -> Fut + Send + 'a>(f: F) -> Self {
        FnRouter { inner: Box::new(f) }
    }
}

impl<P, Fut> Router for FnRouter<'_, P, Fut>
where
    P: AsyncRead + AsyncWrite + Send,
    Fut: Future<Item = P, Error = Error>,
{
    type Item = P;
    type Fut = Fut;

    fn route(&mut self, endpoint: &Endpoint) -> Self::Fut {
        (self.inner)(endpoint)
    }
}
