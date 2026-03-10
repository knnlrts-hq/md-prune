> ***One American dies of melanoma almost every hour.**

**Rather, if you take the sentence literally, it does not at all mean what the AMF intended. What the sentence actually claims is that there is one American, Person X, who has the misfortune—to say nothing of the remarkable ability of almost instant resurrection—to die of melanoma every hour.**

**When writers and speakers use language in everyday contexts to talk about everyday circumstances, they and their readers and listeners almost always share a common knowledge of the world (and in particular what is being written or spoken about), and that common knowledge can be drawn upon to determine the intended meaning. But when mathematicians (and scientists) use language in their work, there often is limited or no shared, common understanding—everyone is engaged in a process of discovery. Moreover, in mathematics, the need for precision is paramount. As a result, when mathematicians use language in doing mathematics, they rely on the literal meaning. Which means, of course, they have to be aware of the literal meaning of what they write or say.**

# 2.1 Mathematical statements
**Modern, pure mathematics is primarily concerned with statements about mathematical objects. Mathematical objects are things such as: integers, real numbers, sets, functions, etc.**

- **(1) There are infinitely many prime numbers.**
- **(2) For every real number $a$, the equation $x^2 + a = 0$ has a real root.**
- **(3) $\sqrt{2}$ is irrational.**
- **(4) If $p(n)$ denotes the number of primes less than or equal to the natural number $n$, then as $n$ becomes very large, $p(n)$ approaches $n / \log_{\mathrm{e}}n$.**

**Not only are mathematicians interested in statements of the above kind, they are, above all, interested in knowing which statements are true and which are false. For instance, in the above examples, (1), (3), and (4) are true whereas (2) is false. The truth or falsity in each case is demonstrated not by observation, measurement, or experiment, as in the sciences, but by a proof**

$$
p_{1}, \ p_{2}, \ p_{3}, \ \dots , \ p_{n}, \ \dots
$$

$$
p_{1}, \ p_{2}, \ p_{3}, \ \dots, \ p_{n}
$$

**The goal is to show that there is another prime that can be added to the list. Provided we do this without assigning $n$ a specific value, this will imply at once that the list is infinite.**

**Let $N$ be the number we get when we multiply together all the primes we have listed so far and then add 1, i.e.,**

$$
N = (p _{1} \cdot p_{2} \cdot p_{3} \cdot \ldots \cdot p_{n}) + 1
$$

**Obviously, $N$ is bigger than all the primes in our list, so if $N$ is prime, we know there is a prime bigger than $p_n$, and hence the list can be continued. (We are not saying that $N$ is that next prime. In fact, $N$ will be much bigger than $p_n$, so it is unlikely to be the next prime.)**

**Now let's see what happens if $N$ is not prime. Then there must be a prime $q < N$ such that $q$ divides $N$. But none of $p_1, \ \ldots, \ p_n$ divides $N$, since the division of $N$ by any one of these leaves a remainder of 1. So, $q$ must be bigger than $p_n$. Thus, once again we see that there is a prime bigger than $p_n$, so the list can be continued.**

**Example (2) can easily be proved to be false. Since the square of no real number is negative, the equation $x^{2} + 1 = 0$ does not have a real root. Because there is at least one value of $a$ for which the equation $x^{2} + a = 0$ does not have a real root (namely $a = 1$), we can conclude that statement (2) is false.**

**In everyday life, we use context and our general knowledge of the world and of our lives to fill in missing information in what is written or said, and to eliminate the false interpretations that can result from ambiguities.**

**At first, it might seem like a herculean task to make the use of language in mathematics sufficiently precise. Fortunately, it is possible because of the special, highly restricted nature of mathematical statements. Every key statement of mathematics (the axioms, conjectures, hypotheses, and theorems) is a positive or negative version of one of four linguistic forms:**

- **(1) Object $a$ has property $P$**
- **(2) Every object of type $T$ has property $P$**
- **(3) There is an object of type $T$ having property $P$**
- **(4) If STATEMENT A, then STATEMENT B**

**or else is a simple combination of sub-statements of these forms using the connecting words (combinators): 'and', 'or', and 'not'.**

**The ancient Greek mathematicians seem to have been the first to notice that all mathematical statements can be expressed using one of these simple forms, and they made a systematic study of the language involved, namely the terms 'and' ( $\wedge$ ), 'or' ( $\vee$ ), 'not' ( $\neg$ ), 'implies' ( $\Rightarrow$ ), 'for all' ( $\forall$ ), and 'there exists' ( $\exists$ ). They provided universally accepted meanings of these key terms and analyzed their behavior. Today this formal mathematical study is known as formal logic or mathematical logic.**

# 2.2 The logical combinators and ( $\wedge$ ), or ( $\vee$ ), and not ( $\neg$ )
**As a first step in becoming more precise about our use of language (in mathematical contexts), we shall develop precise, unambiguous definitions of the key connecting words and, or, and not.**

## The combinator and ( $\wedge$ )
**We need to be able to combine two claims into one that asserts both.**

$$
\wedge \ , \ \&
$$

$$
(\pi > 3) \wedge (\pi <   3. 2)
$$

**There is no possible source of confusion when we use the word and. If $\phi$ and $\psi$ are any two mathematical statements,**

$$
\phi \wedge \psi
$$

**is the joint assertion (which may or may not be a valid assertion) of both $\phi$ and $\psi$.** **The symbol $\wedge$ is called wedge, but the expression $\phi \wedge \psi$ is usually read as "$\phi$ and $\psi$."**

**The joint statement $\phi \wedge \psi$ (or $\phi \ \& \ \psi$) is called the conjunction of $\phi$ and $\psi$, and $\phi, \psi$ are called the conjuncts of the combined statement. Introducing formally defined terms to discuss words and concepts we introduce in mathematics, as here, is a common practice. It is impossible to introduce precision without being able to use agreed upon terminology. Likewise, legal contracts frequently have a whole section where the meanings of various terms are stipulated.**

**Notice that if $\phi$ and $\psi$ are both true, then $\phi \wedge \psi$ will be true. But if one or both of $\phi$, $\psi$ is false, then so is $\phi \wedge \psi$. In other words, both conjuncts have to be true in order for the conjunction to be true.**

$$
a <   x \leq b
$$

$$
(a <   x) \wedge (x \leq b).
$$

## The combinator or ( $\vee$ )
**We wish to be able to assert that statement $A$ is true or statement $B$ is true.**

$$
a > 0 \text {o r} x ^ {2} + a = 0 \text {h a s a r e a l r o o t}
$$

$$
a b = 0 \mathrm {i f} a = 0 \mathrm {o r} b = 0
$$

**But mathematics has no place for potential ambiguity in the meaning of such a commonly used word as or, so we must choose one or the other meaning.** **It turns out to be more convenient to opt for the inclusive use.** **Accordingly, whenever we use the word or in mathematics we always mean the inclusive or.** **If $\phi$ and $\psi$ are mathematical statements, $\phi$ or $\psi$ asserts that at least one of $\phi$, $\psi$ is valid.**

![image](https://cdn-mineru.openxlab.org.cn/result/2026-03-05/a9d76dea-69c3-4179-8e6c-efd66b202eea/97c9a75398c8a8569bed740b3b0a62d46b940689529d5b955c2e8f904603a161.jpg)

$$
\phi \wedge \psi
$$

**means that at least one of $\phi, \psi$ is valid.**

**We call $\phi \lor \psi$ the disjunction of $\phi$ and $\psi$, and refer to each of $\phi, \psi$ as the disjuncts of the combined statement.**

**It requires only one of $\phi, \psi$ to be true in order for the disjunction $\phi \lor \psi$ to be true.**

$$
(3 <   5) \vee (1 = 0)
$$

## The combinator not ( $\neg$ )
**Many mathematical statements involve a negation, i.e., a claim that a particular statement is false.**

$$
n o t - \phi
$$

**is the statement that $\psi$ is false.**

**Thus, if $\psi$ is a true statement, not-$\psi$ is a false statement, and if $\psi$ is a false statement, not-$\psi$ is a true statement.**

$$
\urcorner \psi
$$

$$
x \neq y
$$

$$
\neg (x = y)
$$

$$
\neg (a <   x \leq b)
$$

$$
a \neq x \not \leq b
$$

**Although the mathematical usage of the word not accords with most common usage, negation is sometimes used very loosely in everyday speech, so you have to be careful.**

$$
\neg (\pi <   3).
$$

$$
\pi \geq 3
$$

$$
(\pi = 3) \vee (\phi > 3).
$$

**All foreign cars are badly made.**

**What is the negation of this statement?**

- **(a) All foreign cars are well made.**
- **(b) All foreign cars are not badly made.**
- **(c) At least one foreign car is well made.**
- **(d) At least one foreign car is not badly made.**

**A common mistake is for the beginner to choose** **(a).** **But this is easily seen to be wrong.** **The original statement is surely false.** **(a) is certainly not true!** **Neither is** **(b) true.** **So realistic considerations lead us to conclude that if the correct answer is to be found in the above list, then it has to be either** **(c) or** **(d).**

**In point of fact, both** **(c) and** **(d) can be said to represent the negation of our original statement.**

**We shall return to this example later, but before we leave it for now, let us note that the original statement is only concerned with foreign cars.** **Hence its negation will only deal with foreign cars.** **So, the negation will not involve any reference to domestic cars.**

**cannot be the negation of our original statement.** **Indeed, knowing whether our original statement is true or false in no way helps us to decide the truth or falsity of the above statement.** **To be sure, domestic is the negation of foreign in this context, but we are negating the assertion as a whole, not some adjective occurring in it.**

**Now you might appreciate why it is important to analyze the use of language before we use it in mathematics.**

# 2.3 Implication ( $\Rightarrow$ )
**Now things get really tricky.** **Brace yourself for several days of confusion until the ideas sort themselves out in your mind.**

**It would not be unreasonable to assume it means the following:**

**If $\phi$ is true, then $\psi$ has to be true as well.**

**But the carefully crafted, lawyer-like wording I used to introduce that possible meaning should indicate that things are slippery.**

**Suppose that for $\phi$ we take the true assertion $\sqrt{2}$ is irrational' (we'll prove that later on) and for $\psi$ we take the true assertion $0 < 1$.** **Then is the expression (*) true?** **In other words, does the irrationality of $\sqrt{2}$ imply that 0 is less than 1?** **Of course it does not.** **There is no meaningful connection between the two statements $\phi$ and $\psi$ in this case.**

**The point is, implies entails causality.** **This was not a consideration in the case of and and or.** **There is nothing to stop us conjoining or disjoining two totally unrelated statements.**

$$
\begin{array}{l} \left(J u l i u s \mathrm {C a e s a r i s d e a d}\right) \wedge (1 + 1 = 2) \\ (J u l i u s C a e s a r i s d e a d) \vee (1 + 1 = 2) \\ \end{array}
$$

**Thus, in adopting precise meanings for the words and, or, and not, we were able to ignore the meanings of the component statements and focus entirely on their truth values (i.e., are the statements true or false?).**

**In the process, we did of course have to make choices which gave some of the terms meanings that were different from their everyday language counterparts.** **We had to stipulate that or means inclusive-or and adopt a minimalistic interpretation of not reminiscent of the "not proven" verdict in a court of law.**

**We will have to adopt a similar approach to implies in order to come up with an unambiguous meaning that depends only on truth and falsity.**

**As I noted once already, the problem is that when we say “$\phi$ implies $\psi$”, we mean that $\phi$ somehow causes or brings about $\psi$.** **This entails that the truth of $\psi$ follows from the truth of $\phi$, but truth alone does not fully capture what is meant by the word “implies”.** **Not even close.**

**The approach we shall adopt is to separate the notion of implication into two parts, the truth part and the causation part.** **The truth part is generally known as the conditional, or sometimes the material conditional.** **Thus we have the relationship:**

$$
\text{implication } = \text{ conditional } + \text{ causation}
$$

$$
\phi \Rightarrow \psi
$$

$$
\phi \Rightarrow \psi
$$

## 2.3.1 Exercises
**If we know that the statement $N > 7$ is true, then we can conclude that $N^2 > 40$ is true.** **According to the first row of our table,**

$$
(N > 7) \Rightarrow (N ^ {2} > 4 0)
$$

**is true.** **This is entirely consistent with the validity of the genuine (causal) implication: $N > 7$ implies $N^2 > 40$.**

**But what happens if $\phi$ is the true statement "Julius Caesar is dead" and $\psi$ is the true statement “$\pi > 3$?”** **According to the first row of our table, the conditional**

$$
(J u l i u s C a e s a r i s d e a d) \Rightarrow (\pi > 3)
$$

**In real-world terms, there is of course no relationship between the true fact that Julius Caesar is dead and the true fact that $\pi$ is greater than 3.** **But so what?** **The conditional does not claim to capture causality relationships, or indeed meaningful relationships of any kind.** **The truth of [(Julius Caesar is dead) $\Rightarrow (\pi > 3)]$ only becomes problematic if you interpret the conditional $(\Rightarrow)$ as implication.**

**Now let's continue with the task of filling in the truth table for the conditional.** **If $\phi$ is true and $\psi$ is false, then there is no way that $\phi$ can genuinely imply $\psi$.** **(Why? Well, if there were a genuine implication, then the truth of $\psi$ would follow automatically from the truth of $\phi$.)** **So if $\phi$ is true and $\psi$ is false, the genuine implication must be false.** **Hence the conditional $[\phi \Rightarrow \psi]$ should be false as well, and the table now looks like this:**

## 2.3.2 Exercises
**At this point, you should probably go back to the start of the discussion of implies and re-read everything we have done so far.**

**Though the use of simple (and often silly) examples can give the impression of it all being an irrelevant game, the consequences are far from irrelevant.** **The next time you step into an airplane, be aware that the flight control software on which your life depends makes use of the formal notions of $\Lambda$, $\vee$, $\neg$, and $\Rightarrow$ we are discussing here.** **And part of what it takes to make that software reliable is that the system never encounters a mathematical statement whose truth is not defined.** **You, as a human being, only care about statements of the form $[\phi \Rightarrow \psi]$ when everything makes sense.** **But computer systems do not have a notion of "making sense."** **They deal in the binary logic of truth and falsity.** **What matters to a computer system is that everything is always precisely defined, with a specific truth value.**

**Once we get used to ignoring all questions of causality, the truth-values of the conditional seem straightforward enough when the antecedent is true.**

**Well, in terms of truth values, $\phi$ will not imply $\psi$ if it is the case that although $\phi$ is true, $\psi$ is nevertheless false.**

**Please read that last sentence again.** **Now once more.** **Okay, now we are ready to continue.**

**We therefore define $\phi \neq \psi$ to be true precisely in case $\phi$ is true and $\psi$ is false.**

**Having defined the truth or falsity of $\phi \neq \psi$, we obtain that of $\phi \Rightarrow \psi$ by just taking the negation.** **The conditional $\phi \Rightarrow \psi$ will be true exactly when $\phi \neq \psi$ false.**

**Examination of this definition leads to the conclusion: $\phi \Rightarrow \psi$ will be true whenever one of the following holds:**

- **(1) $\phi$ and $\psi$ are both true.**
- **(2) $\phi$ is false and $\psi$ is true.**
- **(3) $\phi$ and $\psi$ are both false.**

**The complete truth table thus looks like this:**

**The points to note about this are:**

- **(a) We are defining a notion (the conditional) that captures only part of what 'implies' means.**
- **(b) To avoid difficulties, we base our definition solely on the notion of truth and falsity.**
- **(c) Our definition agrees with our intuition concerning implication in all meaningful cases.**
- **(d) The definition for a true antecedent is based on an analysis of the truth-values of genuine implication.**
- **(e) The definition for a false antecedent is based on a truth-value analysis of the notion that $\phi$ does not imply $\psi$.**

**Summing up, in defining the conditional the way we do, we do not end up with a notion that contradicts the notion of (genuine) implication.** **Rather, we obtain a notion that extends (genuine) implication to cover those cases where a claim of implication is irrelevant (the antecedent is false) or meaningless (there is no real connection between the antecedent and the consequent).** **In the meaningful case where there is a relationship between $\phi$ and $\psi$ and in addition $\phi$ is true, namely the cases covered by the first two rows of the table, the truth value of the conditional will be the same as the truth value of the actual implication.**

**Remember, it is the fact that the conditional always has a well-defined truth value that makes this notion important in mathematics, since in mathematics (as well as in aircraft control systems!) we cannot afford to have statements with undefined truth values floating around.**

## 2.3.3 Exercises
**Closely related to implication is the notion of equivalence.** **Two statements $\phi$ and $\psi$ are said to be (logically) equivalent if each implies the other.** **The analogous, formal notion defined in terms of the conditional is known as the biconditional.**

$$
\phi \Leftrightarrow \psi
$$

**(Modern logic texts use the notation $\phi \leftrightarrow \psi$.)**

$$
(\phi \Rightarrow \psi) \land (\psi \Rightarrow \phi)
$$

**Looking back at the definition of the conditional, this means that the biconditional $\phi \Leftrightarrow \psi$ will be true if $\phi$ and $\psi$ are both true or both false, and $\phi \Leftrightarrow \psi$ will be false if exactly one of $\phi, \psi$ is true and the other false.**

**One way to show that two logical expressions are biconditionally-equivalent is to show that their truth tables are the same.**

**The final column is the same as that for $\phi \Rightarrow \psi$.** **Hence, $(\phi \land \psi) \lor (\neg \phi)$ is biconditionally equivalent to $\phi \Rightarrow \psi$.**

## 2.3.4 Exercises
$$
(\phi \Rightarrow \psi) \Leftrightarrow (\neg \phi \lor \psi)
$$

$$
(\phi \neq \psi) \Leftrightarrow (\phi \wedge \neg \psi)
$$

$$
[ \phi \wedge (\phi \Rightarrow \psi) ] \Rightarrow \psi
$$

**In an implication**

**$\phi$ implies $\psi$**

**we call $\phi$ the antecedent and $\psi$ the consequent.**

**The following all mean the same:**

- **(1) $\phi$ implies $\psi$**
- **(2) if $\phi$ then $\psi$**
- **(3) $\phi$ is sufficient for $\psi$**
- **(4) $\phi$ only if $\psi$**
- **(5) $\psi$ if $\phi$**
- **(6) $\psi$ whenever $\phi$**
- **(7) $\psi$ is necessary for $\phi$**

**The first four all mention $\phi$ before $\psi$, and of these the first three seem obvious enough.** **But caution is required in the case of (4).** **Notice the contrast between (4) and (5) as far as the order of $\phi$ and $\psi$ is concerned.**

**Likewise, the use of the word necessary in (7) often causes confusion.** **Notice that to say that $\psi$ is a necessary condition for $\phi$ does not mean that $\psi$ on its own is enough to guarantee $\phi$.** **Rather what it says is that $\psi$ will have to hold before there can even be any question of $\phi$ holding.** **For that to be the case, $\phi$ must imply $\psi$.**

![[Drawing 2026-03-09 20.34.25.excalidraw]]

$$
\phi \mathrm {i f f} \psi
$$

$$
\lnot (\phi \land \psi) \mathrm {a n d} (\lnot \phi) \lor (\lnot \psi)
$$

# 2.4 Quantifiers
**There are two more (mutually related) language constructions that are fundamental to expressing and proving mathematical facts, and which mathematicians therefore have to be precise about: the two quantifiers:**

## there exists ( $\exists$ ), for all ( $\forall$ )
**The word quantifier is used in a very idiosyncratic fashion here.**

- **- There is an object $x$ having property $P$**
- **- For all objects $x$, property $P$ holds.**

**I'll take these one at a time.**

$$
\mathrm {T h e} x ^ {2} + 2 x + 1 = 0 \mathrm {h a s a r e a l} \mathrm {r o o t}.
$$

$$
T h e r e e x i s t s a r e a l n u m b e r x s u c h t h a t x ^ {2} + 2 x + 1 = 0.
$$

**Mathematicians use the symbol**

$$
\exists x
$$

**to mean**

$$
t h e r e \mathrm {e x i s t s} a n x \mathrm {s u c h} t h a t \ldots
$$

**Using this notation, the above example would be written symbolically as:**

$$
\exists x [ x ^ {2} + 2 x + 1 = 0 ]
$$

**The symbol $\exists$ is called the existential quantifier.** **As you may have suspected, the back-to-front E comes from the word "Exists".**

**One obvious way to prove an existence statement is to find an object that satisfies the expressed condition.** **In this case, the number $x = -1$ does the trick.** **(It's the only number that does, but one is enough to satisfy an existence claim.)**

**Not all true existence claims are proved by finding a requisite object.**

**Mathematicians have other methods for proving statements of the form $\exists x P(x)$.**

## 2.4.1 Exercises
**The same kind of argument I just outlined to show that the cubic equation $y = x^3 + 3x + 1$ has a real root, can be used to prove the "Wobbly Table Theorem."**

**Sometimes it is not immediately obvious that a statement is an existence assertion.**

$$
\sqrt {2} \mathrm {i s r a t i o n a l}
$$

**expresses an existence claim.**

$$
\text {T h e r e} p \text {a n d} q \text {s u c h t h a t} \sqrt {2} = p / q.
$$

**Using the existential quantifier symbol, we might write this as**

$$
\exists p \exists q (\sqrt {2} = p / q)
$$

**This would be fine provided we specified in advance that the variables $p$ and $q$ refer to whole numbers.**

$$
(\exists p \in \mathcal {N}) (\exists q \in \mathcal {N}) (\sqrt {2} = p / q)
$$

**This uses set-theoretic notation that you are probably familiar with, $\mathcal{N}$ denotes the set of natural numbers (i.e., positive whole numbers) and $p \in \mathcal{N}$ means "$p$ is an element (or member) of the set $\mathcal{N}$."**

**Note that I did not write the above formula as $(\exists p, q \in \mathcal{N})(\sqrt{2} = p / q)$.** **You often see experienced mathematicians writing expressions like this, but it is definitely not recommended for beginners.** **Most mathematical statements involve a whole string of quantifiers, and as we'll see, it can get very tricky manipulating the expression in the course of a mathematical argument, so it is safer to stick to the "one variable per quantifier" rule.**

**The above statement, $(\exists p \in \mathcal{N})(\exists q \in \mathcal{N})(\sqrt{2} = p / q)$, turns out to be false.** **The number $\sqrt{2}$ is not rational.**

**Incidentally, one feature you need to get used to in mastering college mathematics, or more generally what I am calling mathematical thinking, is the length of time you may need to spend on one particular detail.**

**The remaining piece of language we need to examine and make sure we fully comprehend is the universal quantifier, which asserts that something holds for all $x$.**

$$
\forall x
$$

**to mean**

$$
f o r a l l x i t i s t h e c a s e t h a t \dots
$$

**The symbol $\forall$ is just an upside-down A, coming from the word “All”.**

**For example, to say that the square of any real number is greater than or equal to 0, we might write**

$$
\forall x (x ^ {2} \geq 0)
$$

**As before, this would be fine, provided we specified in advance that the variable $x$ refers to real numbers.**

$$
(\forall x \in \mathcal {R}) (x ^ {2} \geq 0)
$$

**We would read this as "For all real numbers $x$, the square of $x$ is greater than or equal to 0."**

**Most statements in mathematics involve combinations of both kinds of quantifier.**

$$
(\forall m \in \mathcal {N}) (\exists n \in \mathcal {N}) (n > m)
$$

**This reads: for all natural numbers $m$ it is the case that there exists a natural number $n$ such that $n$ is greater than $m$.**

**Notice that the order in which quantifiers appear can be of paramount importance.**

$$
(\exists n \in \mathcal {N}) (\forall m \in \mathcal {N}) (n > m)
$$

**This asserts that there is a natural number which exceeds all natural numbers —an assertion that is clearly false!**

**Now it should be clear why we need to avoid using language the way the American Melanoma Foundation writer did when crafting that statement.** **One American dies of melanoma almost every hour.** **That sentence has the logical form**

**$\exists A \forall H [A \text{ dies in hour } H]$**

**when what is meant is**

**$\forall H \exists A [A \text{ dies in hour } H]$.**

## 2.4.2 Exercises
**In mathematics (and in everyday life), you often find yourself having to negate a statement involving quantifiers.** **Of course, you can do it simply by putting a negation symbol in front.** **But often that's not enough; you need to produce a positive assertion, not a negative one.** **The examples I'll give should make it clear what I mean by "positive" here, but roughly speaking, a positive statement is one that says what is, rather than what is not.**

**Let $A(x)$ denote some property of $x$.**

$$
\neg [ \forall x A (x) ] \text {i s e q u i v a l e n t t o} \exists x [ \neg A (x) ]
$$

**For example,**

**It is not the case that all motorists run red lights**

**is equivalent to**

**There is a motorist who does not run red lights.**

**With a familiar example like this, the equivalence is obvious.**

**Now for the abstract verification.**

**That is, we assume it is not the case that $\forall x A(x)$ is true.** **Well, if it is not the case that all $x$ satisfy $A(x)$, what must happen is that at least one of the $x$ must fail to satisfy $A(x)$.** **In other words, for at least one $x$, $\neg A(x)$ must be true.** **In symbols, this can be written $\exists x[\neg A(x)]$.** **Hence $\neg [\forall x A(x)]$ implies $\exists x[\neg A(x)]$.**

**Now suppose $\exists x[\neg A(x)]$ Thus there will be an $x$ for which $A(x)$ fails.** **Hence $A(x)$ does not hold for every $x$.** **(It fails for the $x$ where it fails!)** **In other words, it is false that $A(x)$ holds for all $x$.** **In symbols, $\neg [\forall xA(x)]$.** **Thus $\exists x[\neg A(x)]$ implies $\neg [\forall xA(x)]$.**

**Taken together, the two implications just established produce the claimed equivalence.**

## 2.4.3 Exercises
**Now we are in a position to carry out a proper analysis of our earlier problem about domestic cars, where we want to negate the statement**

**All domestic cars are badly made.**

**Let us formulate this symbolically using the notation of Exercise 2.4.2(5).**

$$
(\forall x \in C) [ D (x) \Rightarrow M (x) ]
$$

**Negating this gives**

$$
(\exists x \in C) \neg [ D (x) \Rightarrow M (x) ]
$$

**(One common cause of confusion. Why do we not say $(\exists x \notin C)$? The answer is that the ‘$\in$ C’ part simply tells us which kind of $x$ we are to consider. Since our original statement concerns domestic cars, so will its negation.)**

**Consider now the part**

$$
\neg [ D (x) \Rightarrow M (x) ]
$$

**We have seen already that this is equivalent to**

$$
D (x) \wedge (\neg M (x))
$$

**Hence for our negated statement (in positive form now) we get**

$$
(\exists x \in C) [ D (x) \land (\neg M (x)) ]
$$

**In words, there is a car that is domestic and is not badly made; i.e., there is a domestic car that is not badly made.**

**We can also obtain this result directly as follows, without going through the above symbolic manipulations.**

**If it is not the case that all domestic cars are badly made, then it must be the case that at least one of them fails to be badly made.**

**The issue discussed above causes problems for enough beginners to warrant some further examples.**

**The first is about natural numbers.**

$$
\forall x [ P (x) \Rightarrow O (x) ]
$$

**This says that all primes are odd, which is false.**

$$
\neg \exists x [ P (x) \Rightarrow O (x) ]
$$

**To get to this form, you start with**

$$
\neg \forall x [ P (x) \Rightarrow O (x) ]
$$

**which is equivalent to**

$$
\exists x \neg [ P (x) \Rightarrow O (x) ]
$$

**and that in turn is equivalent to**

$$
\exists x [ P (x) \neq O (x) ]
$$

**which we can reformulate as**

$$
\exists x [ P (x) \wedge \neg O (x) ]
$$

**Thus the $\forall$ becomes a $\exists$ and the $\Rightarrow$ becomes a $\land$.**

**Viewed as a symbolic procedure, what I did above was move the negation symbol successively inside the expression, adjusting the logical connectives appropriately as I did.** **As you will have suspected, it is possible to write down a list of symbol-manipulation rules for doing this kind of thing.** **That would be useful if you wanted to write a computer program to carry out logical reasoning.**

**If we modify the original sentence to read**

$$
(\forall x > 2) [ P (x) \Rightarrow O (x) ]
$$

**(i.e., all primes greater than 2 are odd, which is true) then the negation of this sentence can be written as**

$$
(\exists x > 2) [ P (x) \wedge \neg O (x) ]
$$

**(i.e., there is an even prime bigger than 2), which is false.**

**One thing to notice about this example is that the quantifier $(\forall x > 2)$ changes to $(\exists x > 2)$, not to $(\exists x \leq 2)$.** **Likewise, negation of the quantifier $(\exists x > 2)$ leads to $(\forall x > 2)$, not to $(\forall x \leq 2)$.** **You should make sure you understand the reason for this behavior.**

## 2.4.4 Exercise
$$
\exists x [ P (x) \land \neg H (x) ]
$$

$$
\forall x [ \neg P (x) \lor H (x) ]
$$

$$
\forall x [ P (x) \Rightarrow H (x) ]
$$

$$
\forall x [ x > 0 \Rightarrow \exists y (x y = 1) ]
$$

$$
\begin{array}{l} \neg \forall x [ x > 0 \Rightarrow \exists y (x y = 1) ] \Leftrightarrow \exists x [ x > 0 \land \neg \exists y (x y = 1) ] \\ \Leftrightarrow \exists x [ x > 0 \wedge \forall y (x y \neq 1) ] \\ \end{array}
$$

$$
\forall x \exists y (y ^ {2} = x)
$$

$$
x \geq 0 \Rightarrow \sqrt {x} \geq 0
$$

$$
(\forall x \in \mathcal {R}) [ x \geq 0 \Rightarrow \sqrt {x} \geq 0 ]
$$

$$
\forall x [ E (x) \lor O (x) ]
$$

$$
\forall x E (x) \lor \forall x O (x)
$$

$$
\exists x [ E (x) \land O (x) ]
$$

$$
\exists x E (x) \land \exists x O (x)
$$

$$
(\forall x \in A), (\exists x \in A)
$$

$$
\forall x [ L (x) \Rightarrow S (x) ]
$$

$$
\forall x [ L (x) \land S (x) ]
$$

$$
\exists x [ H (x) \land S (x) ]
$$

$$
\exists x [ H (x) \Rightarrow S (x) ]
$$

$$
(\forall x \in A) \phi (x)
$$

$$
\forall x [ A (x) \Rightarrow \varphi (x) ]
$$

$$
(\exists x \in \mathcal {A}) \phi (x)
$$

$$
\exists x [ A (x) \land \varphi (x) ]
$$

$$
\begin{array}{l} \lnot [ \forall x \exists y \forall z A (x, y, z) ] \Leftrightarrow \exists x \neg [ \exists y \forall z A (x, y, z) ] \\ \Leftrightarrow \exists x \forall y - [ \forall z A (x, y, z) ] \\ \Leftrightarrow \exists x \forall y \exists z \neg [ A (x, y, z) ] \\ \end{array}
$$

$$
\exists ! x \varphi (x)
$$

$$
\exists x [ \varphi (x) \land \forall y [ \varphi (y) \Rightarrow x = y ] ]
$$

## 2.4.5 Exercises
$$
(\forall \in > 0) (\exists \delta > 0) (\forall x) [ | x - a | <   \delta \Rightarrow | f (x) - f (a) | <   \in ]
$$
