# Rustf2F
Rust Closure: Real Case


Closures

As Rust helps to do generics, let’s build some fully generic codes to understand some points in the very informative work "Finding Closure in Rust", see http://huonw.github.io/blog/2015/05/finding-closure-in-rust/. 

Almost certainly, rust helped to shape closures similar to C++11’s using trait concept. 

Three fundamental traits for closures are used to generate closures. When the call method uses: 

1) &self among its arguments, it's a Fn. 
2) &mut self among its arguments, it's a FnMut.
3) self among its arguments, it's a FnOnce. 

This is clearly covering the definition of a closure: "closure is a function that can directly use variables from the scope in which it is defined, its external-environment". 

A fourth point is not fully clarified for a kind of closure that is only aware about its internal-environment, any function defined by "fn" is fully operating only on its internal arguments, no need to know that much about the external-environment to do the job. 

Let's call this function defined by the key "fn" as self-defined. 

When testing simple codes, I observed that any self-defined function is considered by the compiler as Fn, FnMut and FnOnce. 

The current Rust compiler enables the self-defined function to enjoy the Trinity. 

Simple questions: 

What is the doctrine of this Trinity? 

Is the current Rust compilation Trinity contradictory to any idioms? 

Let's have a workaround, we need a concrete and useful case, let's explore a specific case where the power of Rust generic is used. 

Using closure in Remote Procedure Calls (RPC) is basically or naturally motived by the main point of RPC: a client asks a server to process a request, the server will process the function associated to that request (closure ) to return the suitable response if any. Closures are very suitable for this kind of task. 

We defined, generically, 3 kind of call-execution methods, one using only Fn closure ( having the marker Fn ), second using FnMut closure and the third using only FnOnce closure. We used the same name in the tree call-execution methods, "execution". The current Rust compiler passed all of them without a single warning signs. 

So which one the current Rust compiler will infer from these 3 calls at run time? 

When our code has all the closures, Fn*, and when "execute" used a self-defined function as argument, Rust system at run time used: 

-) The execution function defined by FnOnce, ( I expected actually the one defined by Fn.) 

I kept only Fn and FnMut in the code ( I removed any thing related to FnOnce ) , when "execute" used a self-defined function as argument, Rust system at run time used in this case: 

-) The execution function defined by Fn. 

Any interpretation of this behavior? 
