**personally, I learn best by doing. It’s hard for me to wade through paragraphs full of abstract concepts and really absorb them. But if I’ve coded something, run it, and debugged it, then I get it. Static type systems in particular require rigorous formal reasoning. Hacking on a type system has the same feel as proving a theorem in mathematics. That’s my goal for you. I want you to come away with a solid intuition of how a real language lives and breathes. My hope is that when you read other, more theoretical books later, the concepts there will firmly stick in your mind, adhered to this tangible substrate.**

> [!info] Curry-Howard isomorphism
> - A **logical statement** is a kind of **specification**
> - A **proof** is a **construction** that fulfills that specification
> - A **program** is also a **construction** that fulfills a type specification

# 1.1 Why Learn This Stuff?

## 1.1.1 Little languages are everywhere
**==For every successful general-purpose language, there are a thousand successful niche ones. We used to call them “little languages”, but inflation in the jargon economy led to the name “domain-specific languages”.== These are pidgins tailorbuilt to a specific task. Think application scripting languages, template engines, markup formats, and configuration files.**

![[2026-02-24 12_49_43-CI-ch01.pdf - Foxit PDF Reader.png]]

**Almost every large software project needs a handful of these. When you can, it’s good to reuse an existing one instead of rolling your own.**

## 1.1.2 Languages are great exercise
**Long distance runners sometimes train with weights strapped to their ankles or at high altitudes where the atmosphere is thin. When they later unburden themselves, the new relative ease of light limbs and oxygen-rich air enables them to run farther and faster. ==Implementing a language is a real test of programming skill. The code is complex and performance critical. You must master recursion, dynamic arrays, trees, graphs, and hash tables. You probably use hash tables at least in your day-to-day programming, but do you really understand them?== Well, after we’ve crafted our own from scratch, I guarantee you will.** **When I did finally start cobbling together my own little interpreters, I quickly learned that, of course, there is no magic at all. It’s just code, and the people who hack on languages are just people. There are a few techniques you don’t often encounter outside of languages, and some parts are a little difficult. But not more difficult than other obstacles you’ve overcome.**

# 1.2 How the Book Is Organized

## 1.2.1 The code
**==Many other language books and language implementations use tools like Lex and Yacc, so-called compiler-compilers, that automatically generate some of the source files for an implementation from some higher-level description. There are pros and cons to tools like those, and strong opinions—some might say religious convictions—on both sides. Yacc is a tool that takes in a grammar file and produces a source file for a compiler, so it’s sort of like a “compiler” that outputs a compiler, which is where we get the term “compiler-compiler”.== Yacc wasn’t the first of its ilk, which is why it’s named “Yacc”—Yet Another Compiler-Compiler. A later similar tool is Bison, named as a pun on the pronunciation of Yacc like “yak”.**

![[8e3146de2bad49f6c3d47e14ad9624fdf9f6f8b9c9ed693f912c4bbf52d91ab8.jpg]]

**We will abstain from using them here. I want to ensure there are no dark corners where magic and confusion can hide, so we’ll write everything by hand. As you’ll see, it’s not as bad as it sounds, and it means you really will understand each line of code and how both interpreters work.** **While the book contains every line of code and teaches what each means, it does not describe the machinery needed to compile and run the interpreter. I assume you can slap together a makefile or a project in your IDE of choice in order to get the code to run.**

## 1 . 2 . 4 Challenges
**Each chapter ends with a few exercises.** **They force you to step off the guided path and explore on your own. They will make you research other languages, figure out how to implement features, or otherwise get you out of your comfort zone. Vanquish the challenges and you’ll come away with a broader understanding and possibly a few bumps and scrapes.**

## 1 . 2 . 5 Design notes
**At some point, you find yourself designing a new language. Once you start playing that game, then the softer, human side of the equation becomes paramount. Things like which features are easy to learn, how to balance innovation and familiarity, what syntax is more readable and to whom.**

# 1.3 The First Interpreter
**The book uses Java and C, but readers have ported the code to many other languages. If the languages I picked aren’t your bag, take a look at those. ==Java is a great language for this. It’s high level enough that we don’t get overwhelmed by fiddly implementation details, but it’s still pretty explicit. Unlike in scripting languages, there tends to be less complex machinery hiding under the hood, and you’ve got static types to see what data structures you’re working with. I also chose Java specifically because it is an object-oriented language.== That paradigm swept the programming world in the ’90s and is now the dominant way of thinking for millions of programmers.**

**GCC and LLVM are written in C++ , as are most JavaScript virtual machines. ==Object-oriented languages are ubiquitous, and the tools and compilers for a language are often written in the same language. A compiler reads files in one language, translates them, and outputs files in another language. You can implement a compiler in any language, including the same language it compiles, a process called self-hosting. If your new compiler is written in its own language, there’s an obvious problem at first: how do you compile it before your compiler exists? You can’t compile your compiler using itself yet, but if you have another compiler for your language written in some other language, you use that one to compile your compiler once. Now you can use the compiled version of your own compiler to compile future versions of itself, and you can discard the original one compiled from the other compiler. This is called bootstrapping==**

![[3c3f9d9d40208f2d8db0495ffdc2318160781adb21ba045e6ae3f7c1db5fb43b.jpg]]

> [!info] Compiler vs transpiler
> **A compiler translates code from one language level to another, usually from a high-level language to lower-level code that a machine or runtime can execute.**
> **A transpiler translates code from one high-level language to another high-level language at roughly the same level of abstraction.** In the broad computer-science sense, a transpiler is really just a kind of compiler—specifically, a source-to-source compiler.

# 1.4 The Second Interpreter
**C is the perfect language for understanding how an implementation really works, all the way down to the bytes in memory and the code flowing through the CPU.** **==In our C interpreter, `clox`, we are forced to implement for ourselves all the things Java gave us for free. We’ll write our own dynamic array and hash table. We’ll decide how objects are represented in memory, and build a garbage collector to reclaim them.==** **Our Java implementation was focused on being correct. Now that we have that down, we’ll turn to also being fast. Our C interpreter will contain a compiler that translates Lox to an efficient bytecode representation, which it then executes. This is the same technique used by implementations of Lua, Python, Ruby, PHP, and many other successful languages.**
