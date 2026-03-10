The American Melanoma Foundation, in its 2009 Fact Sheet, states that:

> ***One American dies of melanoma almost every hour.***

To a mathematician, such a claim inevitably raises a chuckle, and occasionally a sigh. Not because mathematicians lack sympathy for a tragic loss of life. **Rather, if you take the sentence literally, it does not at all mean what the AMF intended. What the sentence actually claims is that there is one American, Person X, who has the misfortune—to say nothing of the remarkable ability of almost instant resurrection—to die of melanoma every hour.** The sentence the AMF writer should have written is

> *Almost every hour, an American dies of melanoma.*

Such misuse of language is fairly common, so much so that arguably it is not really misuse. Everyone reads the first sentence as having the meaning captured accurately by the second. Such sentences have become figures of speech. Apart from mathematicians and others whose profession requires precision of statements, hardly anyone ever notices that the first sentence, when read literally, actually makes an absurd claim.

**When writers and speakers use language in everyday contexts to talk about everyday circumstances, they and their readers and listeners almost always share a common knowledge of the world (and in particular what is being written or spoken about), and that common knowledge can be drawn upon to determine the intended meaning. But when mathematicians (and scientists) use language in their work, there often is limited or no shared, common understanding—everyone is engaged in a process of discovery. Moreover, in mathematics, the need for precision is paramount. As a result, when mathematicians use language in doing mathematics, they rely on the literal meaning. Which means, of course, they have to be aware of the literal meaning of what they write or say.**

This is why beginning students of mathematics in college are generally offered a crash course in the precise use of language. This may sound like a huge undertaking, given the enormous richness and breadth of everyday language. But the language used in mathematics is so constrained that the task actually turns out to be relatively small. The only thing that makes it difficult is that the student has to learn to eliminate the sloppiness of expression that we are familiar with in everyday life, and instead master a highly constrained, precise (and somewhat stylized) way of writing and speaking.

# 2.1 Mathematical statements

**Modern, pure mathematics is primarily concerned with statements about mathematical objects. Mathematical objects are things such as: integers, real numbers, sets, functions, etc.** Examples of mathematical statements are:

- **(1) There are infinitely many prime numbers.**
- **(2) For every real number $a$, the equation $x^2 + a = 0$ has a real root.**
- **(3) $\sqrt{2}$ is irrational.**
- **(4) If $p(n)$ denotes the number of primes less than or equal to the natural number $n$, then as $n$ becomes very large, $p(n)$ approaches $n / \log_{\mathrm{e}}n$.**

**Not only are mathematicians interested in statements of the above kind, they are, above all, interested in knowing which statements are true and which are false. For instance, in the above examples, (1), (3), and (4) are true whereas (2) is false. The truth or falsity in each case is demonstrated not by observation, measurement, or experiment, as in the sciences, but by a proof**, about which I will write more in due course.

The truth of (1) can be proved by an ingenious argument known to Euclid. This proof uses basic facts about prime numbers that will be introduced in chapter 4, but most readers are likely to be familiar with what is required. The idea is to show that if we list the primes in increasing order as:

$$
p_{1}, \ p_{2}, \ p_{3}, \ \dots , \ p_{n}, \ \dots
$$

then the list must continue for ever. (The first few members of the sequence are:  $p_1 = 2, \ p_2 = 3, \ p_3 = 5, \ p_4 = 7, \ p_5 = 11, \ \ldots$)

Consider the list up to some stage $n$:

$$
p_{1}, \ p_{2}, \ p_{3}, \ \dots, \ p_{n}
$$

**The goal is to show that there is another prime that can be added to the list. Provided we do this without assigning $n$ a specific value, this will imply at once that the list is infinite.**

**Let $N$ be the number we get when we multiply together all the primes we have listed so far and then add 1, i.e.,**

$$
N = (p _{1} \cdot p_{2} \cdot p_{3} \cdot \ldots \cdot p_{n}) + 1
$$

**Obviously, $N$ is bigger than all the primes in our list, so if $N$ is prime, we know there is a prime bigger than $p_n$, and hence the list can be continued. (We are not saying that $N$ is that next prime. In fact, $N$ will be much bigger than $p_n$, so it is unlikely to be the next prime.)**

**Now let's see what happens if $N$ is not prime. Then there must be a prime $q < N$ such that $q$ divides $N$. But none of $p_1, \ \ldots, \ p_n$ divides $N$, since the division of $N$ by any one of these leaves a remainder of 1. So, $q$ must be bigger than $p_n$. Thus, once again we see that there is a prime bigger than $p_n$, so the list can be continued.** The expression guarantees a new prime divisor, not necessarily a prime number.

Since the above argument does not depend in any way upon the value of $n$, it follows that there are infinitely many primes.

**Example (2) can easily be proved to be false. Since the square of no real number is negative, the equation $x^{2} + 1 = 0$ does not have a real root. Because there is at least one value of $a$ for which the equation $x^{2} + a = 0$ does not have a real root (namely $a = 1$), we can conclude that statement (2) is false.**

I'll give a proof of (3) later. The only known proofs of (4) are extremely complicated—far too complicated to be included in an introductory text such as this.

Clearly, before we can prove whether a certain statement is true or false, we must be able to understand precisely what the statement says. Above all, mathematics is a very precise subject, where exactness of expression is required. This already creates a difficulty, for words tend to be ambiguous, and in real life our use of language is rarely precise.

In particular, when we use language in an everyday setting, we often rely on context to determine what our words convey. An American can truthfully say "July is a summer month," but that would be false if spoken by an Australian. The word "summer" means the same in both statements (namely the hottest three months of the year), but it refers to one part of the year in America and another in Australia.

To take another example, in the phrase "small rodent" the word "small" means something different (in terms of size) than it does in the phrase "small elephant." Most people would agree that a small rodent is a small animal, but a small elephant is definitely not a small animal. The size range referred to by the word "small" can vary depending on the entity to which it is applied.

**In everyday life, we use context and our general knowledge of the world and of our lives to fill in missing information in what is written or said, and to eliminate the false interpretations that can result from ambiguities.**

For example, we would need to know something about the context in order to correctly understand the statement

> *The man saw the woman with a telescope.*

Who had the telescope, the man or the woman?

Ambiguities in newspaper headlines—which are generally written in great haste—can sometimes result in unintended but amusing second readings. Among my favorites that have appeared over the years are:
- Sisters reunited after ten years in checkout line at Safeway.
- Prostitutes appeal to the Pope.
- Large hole appears in High Street. City authorities are looking into it.
- Mayor says bus passengers should be belted.

To systematically make the English language precise (by defining exactly what each word is to mean) would be an impossible task. It would also be unnecessary, since people generally do just fine by relying on context and background knowledge.

But in mathematics, things are different. Precision is crucial, and it cannot be assumed that all parties have the same contextual and background knowledge in order to remove ambiguities. Moreover, since mathematical results are regularly used in science and engineering, the cost of miscommunication through an ambiguity can be high, possibly fatal.

**At first, it might seem like a herculean task to make the use of language in mathematics sufficiently precise. Fortunately, it is possible because of the special, highly restricted nature of mathematical statements. Every key statement of mathematics (the axioms, conjectures, hypotheses, and theorems) is a positive or negative version of one of four linguistic forms:**

- **(1) Object $a$ has property $P$**
- **(2) Every object of type $T$ has property $P$**
- **(3) There is an object of type $T$ having property $P$**
- **(4) If STATEMENT A, then STATEMENT B**

**or else is a simple combination of sub-statements of these forms using the connecting words (combinators): 'and', 'or', and 'not'.**

For example,

- (1) 3 is a prime number. 10 is not a prime number.
- (2) Every polynomial equation has a complex root. It is not the case that every polynomial equation has a real root.
- (3) There is a prime number between 20 and 25. There is no even number beyond 2 that is prime.
- (4) If $p$ is a prime of the form $4n + 1$, then $p$ is a sum of two squares (e.g. $4 \cdot 3 + 1 = 13 = 2^2 + 3^2$)

The final statement, about the primes of the form $4n + 1$, is a celebrated theorem of Gauss.

In their everyday work, mathematicians often use more fluent variants of these forms, such as "Not every polynomial equation has a real root" or "No even number is prime except for 2." But those are just that: variants.

**The ancient Greek mathematicians seem to have been the first to notice that all mathematical statements can be expressed using one of these simple forms, and they made a systematic study of the language involved, namely the terms 'and' ( $\wedge$ ), 'or' ( $\vee$ ), 'not' ( $\neg$ ), 'implies' ( $\Rightarrow$ ), 'for all' ( $\forall$ ), and 'there exists' ( $\exists$ ). They provided universally accepted meanings of these key terms and analyzed their behavior. Today this formal mathematical study is known as formal logic or mathematical logic.**

Mathematical logic is a well-established branch of mathematics, studied and used in university departments of mathematics, computer science, philosophy, and linguistics. (It gets a lot more complicated than the original work carried out in ancient Greece by Aristotle and his followers and by the Stoic logicians.)

Some mathematics transition courses and course textbooks include a brief tour through the more basic parts of mathematical logic (as did I in my book "*Sets, Functions, and Logic*"). But that is not necessary in order to become adept at mathematical thinking. (Many professional mathematicians have virtually no knowledge of mathematical logic.) Consequently, in this book I shall follow a less formal—but still rigorous—path.

## 2.1.1 Exercises

1. How would you show that not every number of the form $N = (p_{1} \cdot p_{2} \cdot p_{3} \cdot \ldots \cdot p_{n}) + 1$ is prime, where $p_{1}, \ p_{2}, \ p_{3}, \ \ldots, \ p_{n}, \ \ldots$  is the list of all prime numbers?
> [!success] Answer
> By providing a concrete counterexample.
> For example: $N = (2 \cdot 3 \cdot 5 \cdot 7 \cdot 11 \cdot 13) + 1 = 30031$
> Now factor: $30031 = 59 \cdot 509$, so $30031$ is composite, not prime. 
> This proves that not every number of the form $N = (p_{1} \cdot p_{2} \cdot p_{3} \cdot \ldots \cdot p_{n}) + 1$ is prime. Only one counterexample is enough.

2. Find two unambiguous (but natural sounding) sentences equivalent to the sentence "*The man saw the woman with a telescope*", the first where the man has the telescope, the second where the woman has the telescope.
> [!success] Answer

3. For each of the four ambiguous newspaper headlines I stated earlier, rewrite it in a way that avoids the amusing second meaning, while retaining the brevity of a typical headline:
- (a) Sisters reunited after ten years in checkout line at Safeway.
- (b) Prostitutes appeal to the Pope.
- (c) Large hole appears in High Street. City authorities are looking into it.
- (d) Mayor says bus passengers should be belted.
> [!success] Answer

4. The following notice was posted on the wall of a hospital emergency room: "*NO HEAD INJURY IS TOO TRIVIAL TO IGNORE.*" Reformulate to avoid the unintended second reading. (The context for this sentence is so strong that many people have difficulty seeing there is an alternative meaning.)
> [!success] Answer

5. You often see the following notice posted in elevators: "*IN CASE OF FIRE, DO NOT USE ELEVATOR.*" This one always amuses me. Comment on the two meanings and reformulate to avoid the unintended second reading. (Again, given the context for this notice, the ambiguity is not problematic.)
> [!success] Answer

6. Official documents often contain one or more pages that are empty apart from one sentence at the bottom: "*This page intentionally left blank.*" Does the sentence make a true statement? What is the purpose of making such a statement? What reformulation of the sentence would avoid any logical problems about truth? (Once again, the context means that in practice everyone understands the intended meaning and there is no problem. But the formulation of a similar sentence in mathematics at the start of the twentieth century destroyed one prominent mathematician's seminal work and led to a major revolution in an entire branch of mathematics.)
> [!success] Answer

7. Find (and provide citations for) three examples of published sentences whose literal meaning is (clearly) not what the writer intended. (This is much easier than you might think. Ambiguity is very common.)
> [!success] Answer

8. Comment on the sentence "*The temperature is hot today.*" You hear people say things like this all the time, and everyone understands what is meant. But using language in this sloppy way in mathematics would be disastrous.
> [!success] Answer

9. Provide a context and a sentence within that context, where the word and occurs five times in succession, with no other word between those five occurrences. (You are allowed to use punctuation.)
> [!success] Answer

10. Provide a context and a sentence within that context, where the words and, or, and, or, and occur in that order, with no other word between them. (Again, you can use punctuation.)
> [!success] Answer

# 2.2 The logical combinators and ( $\wedge$ ), or ( $\vee$ ), and not ( $\neg$ )

**As a first step in becoming more precise about our use of language (in mathematical contexts), we shall develop precise, unambiguous definitions of the key connecting words and, or, and not.** (The other terms, implies, equivalent, for all, and there exist, are more tricky and we'll handle them later.)

## The combinator and ( $\wedge$ )

**We need to be able to combine two claims into one that asserts both.** For instance, we may wish to say that $\pi$ is greater than 3 and less than 3.2. So the word and is indispensable.

Sometimes, in order to achieve a completely symbolic expression, we introduce an abbreviation for and. The most common ones are

$$
\wedge \ , \ \&
$$

In this book I'll use the former. Thus, the expression

$$
(\pi > 3) \wedge (\pi <   3. 2)
$$

says:

> $\pi$ is greater than 3 and $\pi$ is less than 3.2.

In other words, $\pi$ lies between 3 and 3.2.

**There is no possible source of confusion when we use the word and. If $\phi$ and $\psi$ are any two mathematical statements,**

$$
\phi \wedge \psi
$$

**is the joint assertion (which may or may not be a valid assertion) of both $\phi$ and $\psi$.** **The symbol $\wedge$ is called wedge, but the expression $\phi \wedge \psi$ is usually read as "$\phi$ and $\psi$."**

**The joint statement $\phi \wedge \psi$ (or $\phi \ \& \ \psi$) is called the conjunction of $\phi$ and $\psi$, and $\phi, \psi$ are called the conjuncts of the combined statement. Introducing formally defined terms to discuss words and concepts we introduce in mathematics, as here, is a common practice. It is impossible to introduce precision without being able to use agreed upon terminology. Likewise, legal contracts frequently have a whole section where the meanings of various terms are stipulated.**

**Notice that if $\phi$ and $\psi$ are both true, then $\phi \wedge \psi$ will be true. But if one or both of $\phi$, $\psi$ is false, then so is $\phi \wedge \psi$. In other words, both conjuncts have to be true in order for the conjunction to be true.** It only takes one conjunct to be false in order to render the conjunction false.

One thing to notice is that and is independent of order in mathematics: $\phi \wedge \psi$ means the same thing as $\psi \wedge \phi$. This is not always true when we use and in everyday life. For example,

John took the free kick and the ball went into the net

does not mean the same thing as

The ball went into the net and John took the free kick.

Mathematicians sometimes use special notation to represent a conjunction of two statements. For example, in dealing with real numbers, we usually write, say,

$$
a <   x \leq b
$$

instead of

$$
(a <   x) \wedge (x \leq b).
$$


## 2.2.1 Exercises

- 1. The mathematical concept of conjunction captures the meaning of “and” in everyday language. True or false? Explain your answer.
- 2. Simplify the following symbolic statements as much as you can, leaving your answer in the standard symbolic form. (In case you are not familiar with the notation, I'll answer the first one for you.)

- (a) $(\pi > 0) \wedge (\pi < 10)$ [Answer: $0 < \pi < 10$.]
- (b) $(p \geq 7) \land (p < 12)$
- (c) $(x > 5) \land (x < 7)$
- (d) $(x < 4) \land (x < 6)$
- (e) $(y < 4) \land (y^{2} < 9)$
- (f) $(x\geq 0)\wedge (x\leq 0)$

- 3. Express each of your simplified statements from Question 1 in natural English.
- 4. What strategy would you adopt to show that the conjunction $\phi_1 \wedge \phi_2 \wedge \ldots \wedge \phi_n$ is true?
- 5. What strategy would you adopt to show that the conjunction $\phi_1 \wedge \phi_2 \wedge \ldots \wedge \phi_n$ is false?
- 6. Is it possible for one of $(\phi \wedge \psi) \wedge \theta$ and $\phi \wedge (\psi \wedge \theta)$ to be true and the other false, or does the associative property hold for conjunction? Prove your answer.
- 7. Which of the following is more likely?

- (a) Alice is a rock star and works in a bank.
- (b) Alice is quiet and works in a bank.
- (c) Alice is quiet and reserved and works in a bank.
- (d) Alice is honest and works in a bank.
- (e) Alice works in a bank.

If you believe there is no definite answer, say so.

8. In the following table, $\mathrm{T}$ denotes 'true' and $\mathrm{F}$ denotes 'false'. The first two columns list all the possible combinations of values of $\mathrm{T}$ and $\mathrm{F}$ that the two statements $\phi$ and $\psi$ can have. The third column should give the truth value (T or F) $\phi \land \psi$ achieves according to each assignment of T or F to $\phi$ and $\psi$.

<table style="margin-left: auto; margin-right: auto;">
	<colgroup>
		<col style="width: 120px;">
		<col style="width: 120px;">
		<col style="width: 120px;">
	</colgroup>
	<thead>
		  <tr>
			  <th style="text-align:center;">&phi;</th>
			  <th style="text-align:center;">&psi;</th>
			  <th style="text-align:center;">&phi; &wedge; &psi;</th>
		  </tr>
	</thead>
	<tbody>
		  <tr>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">?</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">?</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">?</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">?</td>
		  </tr>
	</tbody>
</table>


Fill in the final column. The resulting table is an example of a "propositional truth table."


## The combinator or ( $\vee$ )

**We wish to be able to assert that statement $A$ is true or statement $B$ is true.** For instance, we might want to say

$$
a > 0 \text {o r} x ^ {2} + a = 0 \text {h a s a r e a l r o o t}
$$

or perhaps we want to say

$$
a b = 0 \mathrm {i f} a = 0 \mathrm {o r} b = 0
$$

These two simple examples show we face a potential ambiguity. The meaning of or is different in these two cases. In the first assertion there is no possibility of both eventualities occurring at once. Moreover, the meaning is unchanged if we put the word either at the beginning of the sentence. In the second case, it is quite possible for both $a$ and $b$ to be zero. Even if we were to alter our second assertion by inserting the word *either*, we would still read it as allowing for both possibilities to occur at once, since the use of the word *either* only serves to strengthen the exclusivity in an assertion *where it is already clear* that there is no possibility of both, as in the first assertion. 

**But mathematics has no place for potential ambiguity in the meaning of such a commonly used word as or, so we must choose one or the other meaning.** **It turns out to be more convenient to opt for the inclusive use.** **Accordingly, whenever we use the word or in mathematics we always mean the inclusive or.** **If $\phi$ and $\psi$ are mathematical statements, $\phi$ or $\psi$ asserts that at least one of $\phi$, $\psi$ is valid.** We use the symbol

![image](https://cdn-mineru.openxlab.org.cn/result/2026-03-05/a9d76dea-69c3-4179-8e6c-efd66b202eea/97c9a75398c8a8569bed740b3b0a62d46b940689529d5b955c2e8f904603a161.jpg)

to denote (inclusive) or. Thus

$$
\phi \wedge \psi
$$

**means that at least one of $\phi, \psi$ is valid.** The symbol $\vee$ is called vee, but mathematicians usually read $\phi \vee \psi$ as “$\phi$ or $\psi$.”

**We call $\phi \lor \psi$ the disjunction of $\phi$ and $\psi$, and refer to each of $\phi, \psi$ as the disjuncts of the combined statement.**

**It requires only one of $\phi, \psi$ to be true in order for the disjunction $\phi \lor \psi$ to be true.**

For instance, the following (rather silly) statement is true:

$$
(3 <   5) \vee (1 = 0)
$$

Although this is a silly example, you should pause for a moment and make sure you understand why this statement is not only mathematically meaningful but actually true. Silly examples are often useful to help understand tricky concepts—and disjunction can be tricky.


## 2.2.2 Exercises

1. Simplify the following symbolic statements as much as you can, leaving your answer in a standard symbolic form (assuming you are familiar with the notation):

- (a) $(\pi > 3) \vee (\pi > 10)$
- (b) $(x < 0) \vee (x > 0)$
- (c) $(x = 0)\lor (x > 0)$
- (d) $(x > 0)\lor (x\geq 0)$
- (e) $(x > 3) \vee (x^{2} > 9)$

2. Express each of your simplified statements from Question 1 in natural English.

3. What strategy would you adopt to show that the disjunction $\phi_1 \lor \phi_2 \lor$ ... $\vee \phi_{n}$ is true?

- 4. What strategy would you adopt to show that the disjunction $\phi_1 \lor \phi_2 \lor \dots \lor \phi_n$ is false?
- 5. Is it possible for one of $(\phi \wedge \psi) \vee \theta$ or $\phi \vee (\psi \vee \theta)$ to be true and the other false, or does the associative property hold for disjunction? Prove your answer.
- 6. Which of the following is more likely?

- (a) Alice is a rock star or she works in a bank.
- (b) Alice is quiet and works in a bank.
- (c) Alice is a rock star.
- (d) Alice is honest and works in a bank.
- (e) Alice works in a bank.

If you believe there is no definite answer, say so.

7. Fill in the entries in the final column of the following truth table:

<table style="margin-left: auto; margin-right: auto;">
	<colgroup>
		<col style="width: 120px;">
		<col style="width: 120px;">
		<col style="width: 120px;">
	</colgroup>
	<thead>
		  <tr>
			  <th style="text-align:center;">&phi;</th>
			  <th style="text-align:center;">&psi;</th>
			  <th style="text-align:center;">&phi; &vee; &psi;</th>
		  </tr>
	</thead>
	<tbody>
		  <tr>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">?</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">?</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">?</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">?</td>
		  </tr>
	</tbody>
</table>


## The combinator not ( $\neg$ )

**Many mathematical statements involve a negation, i.e., a claim that a particular statement is false.**

If $\psi$ is any statement,

$$
n o t - \phi
$$

**is the statement that $\psi$ is false.** It is called the negation of $\psi$.

**Thus, if $\psi$ is a true statement, not-$\psi$ is a false statement, and if $\psi$ is a false statement, not-$\psi$ is a true statement.** Nowadays, the most commonly used symbolic abbreviation for not-$\psi$ is

$$
\urcorner \psi
$$

but older texts sometimes use $\sim \psi$

In certain circumstances we use special notations for negation. For instance, we generally use the more familiar

$$
x \neq y
$$

instead of

$$
\neg (x = y)
$$

On the other hand, we would probably write

$$
\neg (a <   x \leq b)
$$

instead of

$$
a \neq x \not \leq b
$$

as the latter is ambiguous. (We could make it precise, but it seems rather inegant, and mathematicians don't do it.)

**Although the mathematical usage of the word not accords with most common usage, negation is sometimes used very loosely in everyday speech, so you have to be careful.** For instance, there is no confusion about the meaning of the statement

$$
\neg (\pi <   3).
$$

This clearly means

$$
\pi \geq 3
$$

which, incidentally, is the same as

$$
(\pi = 3) \vee (\phi > 3).
$$

But consider the statement

**All foreign cars are badly made.**

**What is the negation of this statement?** For instance, is it any one of the following?

- **(a) All foreign cars are well made.**
- **(b) All foreign cars are not badly made.**
- **(c) At least one foreign car is well made.**
- **(d) At least one foreign car is not badly made.**

**A common mistake is for the beginner to choose** **(a).** **But this is easily seen to be wrong.** **The original statement is surely false.** Hence the negation of that statement will be true. But **(a) is certainly not true!** **Neither is** **(b) true.** **So realistic considerations lead us to conclude that if the correct answer is to be found in the above list, then it has to be either** **(c) or** **(d).** (We shall later see how we can eliminate (a) and (b) by a formal mathematical argument. )

**In point of fact, both** **(c) and** **(d) can be said to represent the negation of our original statement.** (Any well made foreign car testifies the truth of both (c) and (d). ) Which do you think most closely corresponds to the negation of the original statement?

**We shall return to this example later, but before we leave it for now, let us note that the original statement is only concerned with foreign cars.** **Hence its negation will only deal with foreign cars.** **So, the negation will not involve any reference to domestic cars.** For instance, the statement

> *All domestic cars are well made*

**cannot be the negation of our original statement.** **Indeed, knowing whether our original statement is true or false in no way helps us to decide the truth or falsity of the above statement.** **To be sure, domestic is the negation of foreign in this context, but we are negating the assertion as a whole, not some adjective occurring in it.**

**Now you might appreciate why it is important to analyze the use of language before we use it in mathematics.** In the case of examples about cars, we can use our knowledge of the world to sort out what is true and what is false. But when it comes to mathematics, we often do not have enough background knowledge. The statements we write down may constitute all we know.

## 2.2.3 Exercises

1. Simplify the following symbolic statements as much as you can, leaving your answer in a standard symbolic form (assuming you are familiar with the notation):

- (a) $\neg (\pi > 3.2)$
- (b) $\neg (x < 0)$
- (c) $\neg (x^{2} > 0)$
- (d) $\neg (x = 1)$
- (e)

- 2. Express each of your simplified statements from Question 1 in natural English.
- 3. Is showing that the negation $\neg \phi$ is true the same as showing that $\phi$ is false? Explain your answer.
- 4. Fill in the entries in the final column of the following truth table:

<table style="margin-left: auto; margin-right: auto;">
	<colgroup>
		<col style="width: 120px;">
		<col style="width: 120px;">
	</colgroup>
	<thead>
		  <tr>
			  <th style="text-align:center;">&phi;</th>
			  <th style="text-align:center;">&not;&phi;</th>
		  </tr>
	</thead>
	<tbody>
		  <tr>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">?</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">?</td>
		  </tr>
	</tbody>
</table>

5. Let $D$ be the statement "The dollar is strong", $Y$ the statement "The Yuan is strong," and $T$ the statement "New US–China trade agreement signed". Express the main content of each of the following (fictitious) newspaper headlines in logical notation. (Note that logical notation captures truth, but not the many nuances and inferences of natural language.) Be prepared to justify and defend your answers.

- (a) Dollar and Yuan both strong
- (b) Trade agreement fails on news of weak Dollar
- (c) Dollar weak but Yuan strong, following new trade agreement
- (d) Strong Dollar means a weak Yuan

- (e) Yuan weak despite new trade agreement, but Dollar remains strong
- (f) Dollar and Yuan can't both be strong at same time.
- (g) If new trade agreement is signed, Dollar and Yuan can't both remain strong
- (h) New trade agreement does not prevent fall in Dollar and Yuan
- (i) US–China trade agreement fails but both currencies remain strong
- (j) New trade agreement will be good for one side, but no one knows which.

6. In US law, a trial verdict of “Not guilty” is given when the prosecution fails to prove guilt. This, of course, does not mean the defendant is, as a matter of actual fact, innocent. Is this state of affairs captured accurately when we use “not” in the mathematical sense? (i.e., Do “Not guilty” and “$\neg$ guilty” mean the same thing?) What if we change the question to ask if “Not proven” and “$\neg$ proven” mean the same thing?

7. The truth table for $\neg \neg \phi$ is clearly the same as that for $\phi$ itself, so the two expressions make identical truth assertions. This is not necessarily true for negation in everyday life. For example, you might find yourself saying "I was not displeased with the movie." In terms of formal negation, this has the form $\neg (\neg \text{PLEASED})$, but your statement clearly does not mean that you were pleased with the movie. Indeed, it means something considerably less positive. How would you capture this kind of use of language in the formal framework we have been looking at?

# 2.3 Implication ( $\Rightarrow$ )

**Now things get really tricky.** **Brace yourself for several days of confusion until the ideas sort themselves out in your mind.**

In mathematics we frequently encounter expressions of the form

$(^{*})$ $\phi$ implies $\psi$

Indeed implication provides the means by which we prove statements, starting from initial observations or axioms. The question is, what is the meaning of an assertion of the form $(^{*})?$

**It would not be unreasonable to assume it means the following:**

**If $\phi$ is true, then $\psi$ has to be true as well.**

**But the carefully crafted, lawyer-like wording I used to introduce that possible meaning should indicate that things are slippery.**

**Suppose that for $\phi$ we take the true assertion $\sqrt{2}$ is irrational' (we'll prove that later on) and for $\psi$ we take the true assertion $0 < 1$.** **Then is the expression (*) true?** **In other words, does the irrationality of $\sqrt{2}$ imply that 0 is less than 1?** **Of course it does not.** **There is no meaningful connection between the two statements $\phi$ and $\psi$ in this case.**

**The point is, implies entails causality.** **This was not a consideration in the case of and and or.** **There is nothing to stop us conjoining or disjoining two totally unrelated statements.** For example, there is no difficulty in determining the truth of the statements

$$
\begin{array}{l} \left(J u l i u s \mathrm {C a e s a r i s d e a d}\right) \wedge (1 + 1 = 2) \\ (J u l i u s C a e s a r i s d e a d) \vee (1 + 1 = 2) \\ \end{array}
$$

(Once again, I am using a frivolous example to illustrate a tricky point. Since mathematics is frequently applied to real-world situations, we may well encounter a statement that combines the two domains, mathematics and the real world.)

**Thus, in adopting precise meanings for the words and, or, and not, we were able to ignore the meanings of the component statements and focus entirely on their truth values (i.e., are the statements true or false?).**

**In the process, we did of course have to make choices which gave some of the terms meanings that were different from their everyday language counterparts.** **We had to stipulate that or means inclusive-or and adopt a minimalistic interpretation of not reminiscent of the "not proven" verdict in a court of law.**

**We will have to adopt a similar approach to implies in order to come up with an unambiguous meaning that depends only on truth and falsity.** But in this case we have to be far more aggressive—so much so, that to avoid any possible confusion, we will have to use a different term than “implies”.

**As I noted once already, the problem is that when we say “$\phi$ implies $\psi$”, we mean that $\phi$ somehow causes or brings about $\psi$.** **This entails that the truth of $\psi$ follows from the truth of $\phi$, but truth alone does not fully capture what is meant by the word “implies”.** **Not even close.** So we had better agree to not use the word “implies” unless we mean it.

**The approach we shall adopt is to separate the notion of implication into two parts, the truth part and the causation part.** **The truth part is generally known as the conditional, or sometimes the material conditional.** **Thus we have the relationship:**

$$
\text{implication } = \text{ conditional } + \text{ causation}
$$

We will use the symbol $\Rightarrow$ to denote the conditional operator. Thus,

$$
\phi \Rightarrow \psi
$$

denotes the truth part of $\phi$ implies $\psi$.

(Modern mathematical logic texts generally use a single arrow, $\rightarrow$, instead of $\Rightarrow$, but to avoid confusion with the notation for functions you are likely to meet later in your mathematical education, I'll use the more old-fashioned, double-arrow notation for the conditional.)

Any expression of the form

$$
\phi \Rightarrow \psi
$$

is referred to as a conditional expression, or simply a conditional. We refer to $\phi$ as the antecedent of the conditional and $\psi$ as the consequent.

The truth or falsity of a conditional will be defined entirely in terms of the truth or falsity of the antecedent and the consequent. That is to say, whether or not the conditional expression $\phi \Rightarrow \psi$ is true will depend entirely upon the truth or falsity of $\phi$ and $\psi$, taking no account of whether or not there is any meaningful connection between $\phi$ and $\psi$.

The reason this approach turns out to be a useful one, is that in all cases where there is a meaningful and genuine implication $\phi$ implies $\psi$, the conditional $\phi \Rightarrow \psi$ does accord with that implication.

In other words, it will turn out that our defined notion $\phi \Rightarrow \psi$ fully captures $\phi$ implies $\psi$, whenever there is a genuine implication. But our notion extends beyond that to cover all cases where we know the truth and falsity of $\phi$ and $\psi$ but there is no meaningful connection between the two.

Since we are ignoring causation, which is a highly significant aspect of the notion of implication, our definition may (and in fact will) turn out to have consequences that are at the very least counterintuitive, and possibly even absurd. But those will be restricted to situations where there is no genuine implication.

The task we face, then, is to stipulate rules that will enable us to complete the truth table

<table style="margin-left: auto; margin-right: auto;">
	<colgroup>
		<col style="width: 120px;">
		<col style="width: 120px;">
		<col style="width: 120px;">
	</colgroup>
	<thead>
		  <tr>
			  <th style="text-align:center;">&phi;</th>
			  <th style="text-align:center;">&psi;</th>
			  <th style="text-align:center;">&phi; &Rightarrow; &psi;</th>
		  </tr>
	</thead>
	<tbody>
		  <tr>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">?</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">?</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">?</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">?</td>
		  </tr>
	</tbody>
</table>

The first rule is easy. If there is a valid, genuine implication $\phi$ implies $\psi$, then the truth of $\phi$ implies the truth of $\psi$. So the first row of the table has to have T everywhere:

<table style="margin-left: auto; margin-right: auto;">
	<colgroup>
		<col style="width: 120px;">
		<col style="width: 120px;">
		<col style="width: 120px;">
	</colgroup>
	<thead>
		  <tr>
			  <th style="text-align:center;">&phi;</th>
			  <th style="text-align:center;">&psi;</th>
			  <th style="text-align:center;">&phi; &Rightarrow; &psi;</th>
		  </tr>
	</thead>
	<tbody>
		  <tr>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center; background-color:#d9ffbf;">T</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">?</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">?</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">?</td>
		  </tr>
	</tbody>
</table>

## 2.3.1 Exercises

- 1. Fill in the second row of the truth table.
- 2. Provide a justification of your entry.

Before I complete the second row of the truth table (and thereby tell you the answers to the above exercises—so you should do them before reading on), let's take a look at the consequences of the choice we made in completing the first row the way we did.

**If we know that the statement $N > 7$ is true, then we can conclude that $N^2 > 40$ is true.** **According to the first row of our table,**

$$
(N > 7) \Rightarrow (N ^ {2} > 4 0)
$$

**is true.** **This is entirely consistent with the validity of the genuine (causal) implication: $N > 7$ implies $N^2 > 40$.**

**But what happens if $\phi$ is the true statement "Julius Caesar is dead" and $\psi$ is the true statement “$\pi > 3$?”** **According to the first row of our table, the conditional**

$$
(J u l i u s C a e s a r i s d e a d) \Rightarrow (\pi > 3)
$$

has the value T.

**In real-world terms, there is of course no relationship between the true fact that Julius Caesar is dead and the true fact that $\pi$ is greater than 3.** **But so what?** **The conditional does not claim to capture causality relationships, or indeed meaningful relationships of any kind.** **The truth of [(Julius Caesar is dead) $\Rightarrow (\pi > 3)]$ only becomes problematic if you interpret the conditional $(\Rightarrow)$ as implication.** The cost of defining $[\phi \Rightarrow \psi]$ so it always has a well-defined truth value (which is a mathematically valuable property) is that we have to get used to not reading more into the conditional than is given by the definition.

**Now let's continue with the task of filling in the truth table for the conditional.** **If $\phi$ is true and $\psi$ is false, then there is no way that $\phi$ can genuinely imply $\psi$.** **(Why? Well, if there were a genuine implication, then the truth of $\psi$ would follow automatically from the truth of $\phi$.)** **So if $\phi$ is true and $\psi$ is false, the genuine implication must be false.** **Hence the conditional $[\phi \Rightarrow \psi]$ should be false as well, and the table now looks like this:**

<table style="margin-left: auto; margin-right: auto;">
	<colgroup>
		<col style="width: 120px;">
		<col style="width: 120px;">
		<col style="width: 120px;">
	</colgroup>
	<thead>
		  <tr>
			  <th style="text-align:center;">&phi;</th>
			  <th style="text-align:center;">&psi;</th>
			  <th style="text-align:center;">&phi; &Rightarrow; &psi;</th>
		  </tr>
	</thead>
	<tbody>
		  <tr>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center; background-color:#d9ffbf;">T</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center; background-color:#ffbfbf;">F</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">?</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">?</td>
		  </tr>
	</tbody>
</table>

## 2.3.2 Exercises

- 1. Fill in the third and fourth rows of the truth table.
- 2. Provide justifications for your entries.

(I'll get to the third and fourth rows in a moment, so you should do the above exercise before you read on.)

**At this point, you should probably go back to the start of the discussion of implies and re-read everything we have done so far.** Though it might seem we are making much ado about nothing, this entire discussion is typical of work to provide precise definitions of the fundamental concepts of mathematics.

**Though the use of simple (and often silly) examples can give the impression of it all being an irrelevant game, the consequences are far from irrelevant.** **The next time you step into an airplane, be aware that the flight control software on which your life depends makes use of the formal notions of $\Lambda$, $\vee$, $\neg$, and $\Rightarrow$ we are discussing here.** **And part of what it takes to make that software reliable is that the system never encounters a mathematical statement whose truth is not defined.** **You, as a human being, only care about statements of the form $[\phi \Rightarrow \psi]$ when everything makes sense.** **But computer systems do not have a notion of "making sense."** **They deal in the binary logic of truth and falsity.** **What matters to a computer system is that everything is always precisely defined, with a specific truth value.**

**Once we get used to ignoring all questions of causality, the truth-values of the conditional seem straightforward enough when the antecedent is true.** (If they don't, you should go back and read that discussion yet again. There was a reason I suggested you do that!) But what about the last two rows in the truth table, where the antecedent is false?

To deal with this case, we consider not the notion of implication, but its negation. We extract the causation-free, truth-value part of the statement “$\phi$ does not imply $\psi$”, which I shall write as

$$\phi \centernot\implies \psi$$

Leaving aside all question of whether there is a meaningful causal relation between $\phi$ and $\psi$ and concentrating solely on truth values, how can we be sure that “$\phi$ does not imply $\psi$” is a valid statement? More precisely, how should the truth or falsity of the statement $\phi \neq \psi$ depend upon the truth or falsity of $\phi$ and $\psi$?

**Well, in terms of truth values, $\phi$ will not imply $\psi$ if it is the case that although $\phi$ is true, $\psi$ is nevertheless false.**

**Please read that last sentence again.** **Now once more.** **Okay, now we are ready to continue.** On second thought, maybe you should read it a fourth time just to be sure.

**We therefore define $\phi \neq \psi$ to be true precisely in case $\phi$ is true and $\psi$ is false.**

**Having defined the truth or falsity of $\phi \neq \psi$, we obtain that of $\phi \Rightarrow \psi$ by just taking the negation.** **The conditional $\phi \Rightarrow \psi$ will be true exactly when $\phi \neq \psi$ false.**

**Examination of this definition leads to the conclusion: $\phi \Rightarrow \psi$ will be true whenever one of the following holds:**

- **(1) $\phi$ and $\psi$ are both true.**
- **(2) $\phi$ is false and $\psi$ is true.**
- **(3) $\phi$ and $\psi$ are both false.**

**The complete truth table thus looks like this:**

<table style="margin-left: auto; margin-right: auto;">
	<colgroup>
		<col style="width: 120px;">
		<col style="width: 120px;">
		<col style="width: 120px;">
	</colgroup>
	<thead>
		  <tr>
			  <th style="text-align:center;">&phi;</th>
			  <th style="text-align:center;">&psi;</th>
			  <th style="text-align:center;">&phi; &Rightarrow; &psi;</th>
		  </tr>
	</thead>
	<tbody>
		  <tr>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center; background-color:#d9ffbf;">T</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center; background-color:#ffbfbf;">F</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center; background-color:#d9ffbf;">T</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center; background-color:#d9ffbf;">T</td>
		  </tr>
	</tbody>
</table>

**The points to note about this are:**

- **(a) We are defining a notion (the conditional) that captures only part of what 'implies' means.**
- **(b) To avoid difficulties, we base our definition solely on the notion of truth and falsity.**
- **(c) Our definition agrees with our intuition concerning implication in all meaningful cases.**
- **(d) The definition for a true antecedent is based on an analysis of the truth-values of genuine implication.**
- **(e) The definition for a false antecedent is based on a truth-value analysis of the notion that $\phi$ does not imply $\psi$.**

**Summing up, in defining the conditional the way we do, we do not end up with a notion that contradicts the notion of (genuine) implication.** **Rather, we obtain a notion that extends (genuine) implication to cover those cases where a claim of implication is irrelevant (the antecedent is false) or meaningless (there is no real connection between the antecedent and the consequent).** **In the meaningful case where there is a relationship between $\phi$ and $\psi$ and in addition $\phi$ is true, namely the cases covered by the first two rows of the table, the truth value of the conditional will be the same as the truth value of the actual implication.**

**Remember, it is the fact that the conditional always has a well-defined truth value that makes this notion important in mathematics, since in mathematics (as well as in aircraft control systems!) we cannot afford to have statements with undefined truth values floating around.**


## 2.3.3 Exercises

1. Which of the following are true and which are false?

- (a) $(\phi^2 > 2) \Rightarrow (\phi > 1.4$
- (b) $(\phi^2 < 0)\Rightarrow (\phi = 3)$
- (c) $(\phi^2 > 0) \Rightarrow (1 + 2 = 4)$
- (d) $(\phi >\phi^2)\Rightarrow (\phi = 5)$
- (e) $(e^2\geq 0)\Rightarrow (e <   0)$
- (f) $\neg (5$ is an integer) $\Rightarrow (5^{2}\geq 1)$
- (g) (The area of a circle of radius 1 is $\phi$) $\Rightarrow$ (3 is prime)
- (h) (Squares have three sides) $\Rightarrow$ (Triangles have four sides)
- (i) (Elephants can climb trees) $\Rightarrow$ (3 is irrational)
- (j) (Euclid's birthday was July 4) $\Rightarrow$ (Rectangles have four sides)

2. As in Exercise 2.2.3(5), let $D$ be the statement “The dollar is strong,” $Y$ the statement “The Yuan is strong,” and $T$ the statement “New US–China trade agreement signed.” Express the main content of each of the following (fictitious) newspaper headlines in logical notation. (Remember, logical notation captures truth, but not the many nuances and inferences of natural language.) As before, be prepared to justify and defend your answers.

(a) New trade agreement will lead to strong currencies in both countries.

- (b) If the trade agreement is signed, a rise in the Yuan will result in a fall in the Dollar.
- (c) Dollar weak but Yuan strong, following new trade agreement
- (d) Strong Dollar means a weak Yuan
- (e) New trade agreement means Dollar and Yuan will be tightly linked.

3. Complete the following truth table

<table style="margin-left: auto; margin-right: auto;">
	<colgroup>
		<col style="width: 120px;">
		<col style="width: 120px;">
		<col style="width: 120px;">
		<col style="width: 120px;">
		<col style="width: 120px;">
	</colgroup>
	<thead>
		  <tr>
			  <th style="text-align:center;">&phi;</th>
			  <th style="text-align:center;">&not;&phi;</th>	
			  <th style="text-align:center;">&psi;</th>
			  <th style="text-align:center;">&phi; &Rightarrow; &psi;</th>
			  <th style="text-align:center;">&not;&phi; &vee; &psi;</th>
		  </tr>
	</thead>
	<tbody>
		  <tr>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">?</td>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">?</td>
			  <td style="text-align:center;">?</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">?</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">?</td>
			  <td style="text-align:center;">?</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">?</td>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">?</td>
			  <td style="text-align:center;">?</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">?</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">?</td>
			  <td style="text-align:center;">?</td>
		  </tr>
	</tbody>
</table>

Note: $\neg$ has the same binding rules as $-$ (minus) in arithmetic and algebra, so $\neg \phi \lor \psi$ is the same as $(\neg \phi) \lor \psi$.

- 4. What conclusions can you draw from the above table?
- 5. Complete the following truth table. (Recall that $\phi \neq \psi$ is another way of writing $\neg [\phi \Rightarrow \psi]$.)

<table style="margin-left: auto; margin-right: auto;">
	<colgroup>
		<col style="width: 120px;">
		<col style="width: 120px;">
		<col style="width: 120px;">
		<col style="width: 120px;">
		<col style="width: 120px;">
		<col style="width: 120px;">
	</colgroup>
	<thead>
		  <tr>
			  <th style="text-align:center;">&phi;</th>
			  <th style="text-align:center;">&psi;</th>
			  <th style="text-align:center;">&not;&psi;</th>
			  <th style="text-align:center;">&phi; &Rightarrow; &psi;</th>
			  <th style="text-align:center;">&phi; &nvrArr; &psi;</th>
			  <th style="text-align:center;">&phi; &wedge; &not;&psi;</th>
		  </tr>
	</thead>
	<tbody>
		  <tr>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">?</td>
			  <td style="text-align:center;">?</td>
			  <td style="text-align:center;">?</td>
			  <td style="text-align:center;">?</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">?</td>
			  <td style="text-align:center;">?</td>
			  <td style="text-align:center;">?</td>
			  <td style="text-align:center;">?</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">?</td>
			  <td style="text-align:center;">?</td>
			  <td style="text-align:center;">?</td>
			  <td style="text-align:center;">?</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">?</td>
			  <td style="text-align:center;">?</td>
			  <td style="text-align:center;">?</td>
			  <td style="text-align:center;">?</td>
		  </tr>
	</tbody>
</table>

6. What conclusions can you draw from the above table?

**Closely related to implication is the notion of equivalence.** **Two statements $\phi$ and $\psi$ are said to be (logically) equivalent if each implies the other.** **The analogous, formal notion defined in terms of the conditional is known as the biconditional.** We shall write the biconditional as

$$
\phi \Leftrightarrow \psi
$$

**(Modern logic texts use the notation $\phi \leftrightarrow \psi$.)** The biconditional is formally defined to be an abbreviation for the conjunction

$$
(\phi \Rightarrow \psi) \land (\psi \Rightarrow \phi)
$$

**Looking back at the definition of the conditional, this means that the biconditional $\phi \Leftrightarrow \psi$ will be true if $\phi$ and $\psi$ are both true or both false, and $\phi \Leftrightarrow \psi$ will be false if exactly one of $\phi, \psi$ is true and the other false.**

**One way to show that two logical expressions are biconditionally-equivalent is to show that their truth tables are the same.** Consider, for example, the expression $(\phi \land \psi) \lor (\neg \phi)$. We can build its table column by column as follows:

<table style="margin-left: auto; margin-right: auto;">
	<colgroup>
		<col style="width: 120px;">
		<col style="width: 120px;">
		<col style="width: 120px;">
		<col style="width: 120px;">
		<col style="width: 180px;">
	</colgroup>
	<thead>
		  <tr>
			  <th style="text-align:center;">&phi;</th>
			  <th style="text-align:center;">&psi;</th>	
			  <th style="text-align:center;">&phi; &wedge; &psi;</th>
			  <th style="text-align:center;">&not;&phi;</th>
			  <th style="text-align:center;">(&phi; &wedge; &psi;) &vee; (&not;&phi;)</th>
		  </tr>
	</thead>
	<tbody>
		  <tr>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">T</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">F</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">T</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">T</td>
		  </tr>
	</tbody>
</table>

**The final column is the same as that for $\phi \Rightarrow \psi$.** **Hence, $(\phi \land \psi) \lor (\neg \phi)$ is biconditionally equivalent to $\phi \Rightarrow \psi$.**

We can also draw up tables for expressions involving more than two basic statements, such as $(\phi \wedge \psi) \vee \theta$, which has three, but if there are $n$ constituent statements involved there will be $2^n$ rows in the table, so already $(\phi \wedge \psi) \vee \theta$ needs 8 rows!

## 2.3.4 Exercises

1. Build a truth table to prove the claim I made earlier that $\phi \Leftrightarrow \psi$ is true if $\phi$ and $\psi$ are both true or both false and $\phi \Leftrightarrow \psi$ is false if exactly one of $\phi$, $\psi$ is true and the other false. (To constitute a proof, your table should have columns that show how the entries for $\phi \Leftrightarrow \psi$ are derived, one operator at a time, as in the previous exercises.)

2. Build a truth table to show that

$$
(\phi \Rightarrow \psi) \Leftrightarrow (\neg \phi \lor \psi)
$$

is true for all truth values of $\phi$ and $\psi$. A statement whose truth values are all T is called a logical validity, or sometimes a tautology.

3. Build a truth table to show that

$$
(\phi \neq \psi) \Leftrightarrow (\phi \wedge \neg \psi)
$$

is a tautology.

4. The ancient Greeks formulated a basic rule of reasoning for proving mathematical statements. Called modus ponens, it says that if you know $\phi$ and you know $\phi \Rightarrow \psi$, then you can conclude $\psi$.

(a) Construct a truth table for the logical statement

$$
[ \phi \wedge (\phi \Rightarrow \psi) ] \Rightarrow \psi
$$

(b) Explain how the truth table you obtain demonstrates that modus ponens is a valid rule of inference.

5. Mod-2 arithmetic has just the two numbers 0 and 1 and follows the usual rules of arithmetic together with the additional rule $1 + 1 = 0$. (It is the arithmetic that takes place in a single bit location in a digital computer.) Complete the following table:

<table style="margin-left: auto; margin-right: auto;">
	<colgroup>
		<col style="width: 120px;">
		<col style="width: 120px;">
		<col style="width: 120px;">
		<col style="width: 120px;">
	</colgroup>
	<thead>
		  <tr>
			  <th style="text-align:center;">M</th>
			  <th style="text-align:center;">N</th>
			  <th style="text-align:center;">M &times; N</th>
			  <th style="text-align:center;">M + N</th>
		  </tr>
	</thead>
	<tbody>
		  <tr>
			  <td style="text-align:center;">1</td>
			  <td style="text-align:center;">1</td>
			  <td style="text-align:center;">?</td>
			  <td style="text-align:center;">?</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">1</td>
			  <td style="text-align:center;">0</td>
			  <td style="text-align:center;">?</td>
			  <td style="text-align:center;">?</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">0</td>
			  <td style="text-align:center;">1</td>
			  <td style="text-align:center;">?</td>
			  <td style="text-align:center;">?</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">0</td>
			  <td style="text-align:center;">0</td>
			  <td style="text-align:center;">?</td>
			  <td style="text-align:center;">?</td>
		  </tr>
	</tbody>
</table>

6. In the table you obtained in the above exercise, interpret 1 as T and 0 as F and view M, N as statements.

- (a) Which of the logical combinators $\land, \lor$ corresponds to $x$?
- (b) Which logical combinator corresponds to $+$?
- (c) Does $\neg$ correspond to - (minus)?

- 7. Repeat the above exercise, but interpret 0 as T and 1 as F. What conclusions can you draw?
- 8. The following puzzle was introduced by the psychologist Peter Wason in 1966, and is one of the most famous subject tests in the psychology of reasoning. Most people get it wrong. (So you have been warned!)

Four cards are placed on the table in front of you. You are told (truthfully) that each has a letter printed on one side and a digit on the other, but of course you can only see one face of each. What you see is:

> B E 4 7

You are now told that the cards you are looking at were chosen to follow the rule "If there is a vowel on one side, then there is an odd number on the other side." What is the least number of cards you have to turn over to verify this rule, and which cards do you in fact have to turn over?

There is some terminology associated with implication (i.e., real implication, not just the conditional) that should be mastered straight away, as it pervades all mathematical discussion.

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

**The first four all mention $\phi$ before $\psi$, and of these the first three seem obvious enough.** **But caution is required in the case of (4).** **Notice the contrast between (4) and (5) as far as the order of $\phi$ and $\psi$ is concerned.** Beginners often encounter considerable difficulty in appreciating the distinction between if and only if.

**Likewise, the use of the word necessary in (7) often causes confusion.** **Notice that to say that $\psi$ is a necessary condition for $\phi$ does not mean that $\psi$ on its own is enough to guarantee $\phi$.** **Rather what it says is that $\psi$ will have to hold before there can even be any question of $\phi$ holding.** **For that to be the case, $\phi$ must imply $\psi$.** (This is another of those occasions where my strong advice would be to re-read this paragraph several times until you are sure you get the point—then read it at least one more time!)

The following diagram might help you remember the distinction between 'necessary' and 'sufficient':

![[Drawing 2026-03-09 20.34.25.excalidraw]]

(Think of the word 'sun'. This will remind you of the order.)

Because equivalence reduces to implication both ways, it follows from the above discussion that the following are also equivalent:

- (1) $\phi$ is equivalent to $\psi$
- (2) $\phi$ is necessary and sufficient for $\psi$
- (3) $\phi$ if and only if $\psi$

A common abbreviation for the phrase if and only if is iff (or occasionally iffi). Thus we often write

$$
\phi \mathrm {i f f} \psi
$$

to mean $\phi$ and $\psi$ are equivalent.

Note that if we were to be strict about the matter, the above discussion of equivalent terminologies refers to implication and equivalence, not their formal counterparts the conditional and the biconditional. However, mathematicians frequently use the symbol $\Rightarrow$ as an abbreviation for implies and $\Leftrightarrow$ as an abbreviation for is equivalent to, so the different terminologies often are used together with these formally defined symbols.

Although this is invariably confusing to beginners, it's simply the way mathematical practice has evolved, so there is no getting around it. You would be entirely justified in throwing your hands up at what seems on the face of it to be sloppy practice. After all, if there are genuine problems with the meanings of certain words that necessitate a lengthy discussion and the formulation of formal definitions of concepts that are not identical with their everyday counterparts (such as the difference between the conditional and implication), why do mathematicians then promptly revert to the everyday notions that they began by observing to be problematic?

Here is why the professionals do this: The conditional and biconditional only differ from implication and equivalence in situations that do not arise in the course of normal mathematical practice. In any real mathematical context, the conditional "is" implication and the biconditional "is" equivalence. Thus, having made note of where the formal notions differ from the everyday ones, mathematicians simply move on and turn their attention to other things. (Computer programmers and people who develop aircraft control systems do not have such freedom.)

1. One way to prove that

$$
\lnot (\phi \land \psi) \mathrm {a n d} (\lnot \phi) \lor (\lnot \psi)
$$

are equivalent is to show they have the same truth table:

<table style="margin-left: auto; margin-right: auto;">
	<colgroup>
		<col style="width: 120px;">
		<col style="width: 120px;">
		<col style="width: 120px;">
		<col style="width:120px; background-color:#feffba;">
		<col style="width: 120px;">
		<col style="width: 120px;">
		<col style="width:120px; background-color:#feffba;">
	</colgroup>
	<thead>
		  <tr>
			  <th style="text-align:center;">&phi;</th>
			  <th style="text-align:center;">&psi;</th>
			  <th style="text-align:center;">&phi; &wedge; &psi;</th>
			  <th style="text-align:center;">&not;(&phi; &wedge; &psi;)</th>
			  <th style="text-align:center;">&not;&phi;</th>
			  <th style="text-align:center;">&not;&psi;</th>
			  <th style="text-align:center;">(&not;&phi;) &vee; (&not;&psi;)</th>
		  </tr>
	</thead>
	<tbody>
		  <tr>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">F</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">T</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">T</td>
		  </tr>
		  <tr>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">F</td>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">T</td>
			  <td style="text-align:center;">T</td>
		  </tr>
	</tbody>
</table>

Since the two shaded columns are identical, we know that the two expressions are equivalent.

Thus, negation has the effect that it changes $\vee$ into $\wedge$ and changes $\wedge$ into $\vee$. An alternative way to prove this is to argue directly with the meaning of the first statement:

- 1. $\phi \wedge \psi$ means both $\phi$ and $\psi$ are true.
- 2. Thus $\neg (\phi \land \psi)$ means it is not the case that both $\phi$ and $\psi$ are true.
- 3. If they are not both true, then at least one of $\phi$, $\psi$ must be false.

- 4. This is clearly the same as saying that at least one of $\neg \phi$ and $\neg \psi$ is true. (By the definition of negation).
- 5. By the meaning of or, this can be expressed as $(\neg \phi) \vee (\neg \psi)$.

Provide an analogous logical argument to show that $\neg (\phi \lor \psi)$ and $(\neg \phi) \land (\neg \psi)$ are equivalent.

2. By a denial of a statement $\phi$ we mean any statement equivalent to $\neg \phi$. Give a useful denial of each of the following statements.

- (a) 34,159 is a prime number.
- (b) Roses are red and violets are blue.
- (c) If there are no hamburgers, I'll have a hot-dog.
- (d) Fred will go but he will not play.
- (e) The number $x$ is either negative or greater than 10.
- (f) We will win the first game or the second.

3. Which of the following conditions is necessary for the natural number $n$ to be divisible by 6?

- (a) $n$ is divisible by 3.
- (b) $n$ is divisible by 9.
- (c) $n$ is divisible by 12.
- (d) $n = 24$.
- (e) $n^2$ is divisible by 3.
- (f) $n$ is even and divisible by 3.

- 4. In Exercise 3, which conditions are sufficient for $n$ to be divisible by 6?
- 5. In Exercise 3, which conditions are necessary and sufficient for $n$ to be divisible by 6?
- 6. Let $m, n$ denote any two natural numbers. Prove that $mn$ is odd iff $m$ and $n$ are odd.
- 7. With reference to the previous question, is it true that $mn$ is even iff $m$ and $n$ are even?

8. Show that $\phi \Leftrightarrow \psi$ is equivalent to $(\neg \phi) \Leftrightarrow (\neg \psi)$. How does this relate to your answers to Questions 6 and 7 above?

9. Construct truth tables to illustrate the following:

- (a) $\phi \Leftrightarrow \psi$
- (b) $\phi \Rightarrow (\psi \lor \theta)$

10. Use truth tables to prove that the following are equivalent:

- (a) $\neg (\phi \Rightarrow \psi)$ and $\phi \wedge (\neg \psi)$
- (b) $\phi \Rightarrow (\psi \land \theta)$ and $(\phi \Rightarrow \psi)\land (\phi \Rightarrow \theta)$
- (c) $(\phi \lor \psi)\Rightarrow \theta$ and $(\psi \Rightarrow \theta)\land (\psi \Rightarrow \theta)$

- 11. Verify the equivalences in (b) and (c) in the previous question by means of a logical argument. (So, in the case of (b), for example, you must show that assuming $\phi$ and deducing $\psi \wedge \theta$ is the same as both deducing $\psi$ from $\phi$ and $\theta$ from $\phi$.)
- 12. Use truth tables to prove the equivalence of $\phi \Rightarrow \psi$ and $(\neg \psi) \Rightarrow (\neg \phi)$. $(\neg \psi) \Rightarrow (\neg \phi)$ is called the contrapositive of $\phi \Rightarrow \psi$. The logical equivalence of a conditional and its contrapositive means that one way to prove an implication is to verify the contrapositive. This is a common form of proof in mathematics that we'll encounter later.
- 13. Write down the contrapositives of the following statements:

- (a) If two rectangles are congruent, they have the same area.
- (b) If a triangle with sides $a, b, c$ ($c$ largest) is right-angled, then $a^2 + b^2 = c^2$.
- (c) If $2n - 1$ is prime, then $n$ is prime.
- (d) If the Yuan rises, the Dollar will fall.

- 14. It is important not to confuse the contrapositive of a conditional $\phi \Rightarrow \psi$ with its converse $\psi \Rightarrow \phi$. Use truth tables to show that the contrapositive and the converse of $\phi \Rightarrow \psi$ are not equivalent.
- 15. Write down the converses of the four statements in Question 13.
- 16. Show that for any two statements $\phi$ and $\psi$ either $\phi \Rightarrow \psi$ or its converse is

true (or both). This is another reminder that the conditional is not the same as implication.

17. Express the combinator

 $\phi$ unless $\psi$

in terms of the standard logical combinators.

18. Identify the antecedent and the consequent in each of the following conditionals:

- (a) If the apples are red, they are ready to eat.
- (b) The differentiability of a function $f$ is sufficient for $f$ to be continuous.
- (c) A function $f$ is bounded if $f$ is integrable.
- (d) A sequence $s$ is bounded whenever $s$ is convergent.
- (e) It is necessary that $n$ is prime in order for $2^n - 1$ to be prime.
- (f) The team wins only when Karl is playing.
- (g) When Karl plays the team wins.
- (h) The team wins when Karl plays.

19. Write the converse and contrapositive of each conditional in the previous question.

20. Let $\dot{\vee}$ denote the 'exclusive or' that corresponds to the English expression "either one or the other but not both". Construct a truth table for this connective.

21. Express $\phi \ \dot{\vee} \ \psi$ in terms of the basic combinators $\wedge , \ \vee , \ \neg$

22. Which of the following pairs of propositions are equivalent?

- (a) $\neg (P\lor Q),\neg P\land \neg Q$
- (b) $\neg P \lor \neg Q, \neg (P \lor \neg Q)$
- (c) $\neg (P\land Q),\neg P\lor \neg Q$
- (d) $\neg (P\Rightarrow (Q\land R)),\neg (P\Rightarrow Q)\lor \neg (P\Rightarrow R)$
- (e) $P \Rightarrow (Q \Rightarrow R), (P \land Q) \Rightarrow R$

23. Give, if possible, an example of a true conditional sentence for which

- (a) the converse is true.
- (b) the converse is false.
- (c) the contrapositive is true.
- (d) the contrapositive is false.

24. You are in charge of a party where there are young people. Some are drinking alcohol, others soft drinks. Some are old enough to drink alcohol legally, others are under age. You are responsible for ensuring that the drinking laws are not broken, so you have asked each person to put his or her photo ID on the table. At one table are four young people. One person has a beer, another has a Coke, but their IDs happen to be face down so you cannot see their ages. You can, however, see the IDs of the other two people. One is under the drinking age, the other is above it. Unfortunately, you are not sure if they are drinking Seven-up or vodka and tonic. Which IDs and/or drinks do you need to check to make sure that no one is breaking the law?

25. Compare the logical structure of the previous question with Wason's problem (Exercise 2.3.4(8)). Comment on your answers to those two questions. In particular, identify any logical rules you used in solving each problem, say which one was easier, and why you felt it was easier.


# 2.4 Quantifiers

**There are two more (mutually related) language constructions that are fundamental to expressing and proving mathematical facts, and which mathematicians therefore have to be precise about: the two quantifiers:**

## there exists ( $\exists$ ), for all ( $\forall$ )

**The word quantifier is used in a very idiosyncratic fashion here.** In normal use it means specifying the number or amount of something. In mathematics it is used to refer to the two extremes: there is at least one and for all. The reason for this restricted use is the special nature of mathematical truths. The majority of mathematical theorems—the core of mathematics when viewed as a subject in its own right (as opposed to a set of tools used in other disciplines and walks of life)—are of one of the two forms

- **- There is an object $x$ having property $P$**
- **- For all objects $x$, property $P$ holds.**

**I'll take these one at a time.** A simple example of an existence statement is:

$$
\mathrm {T h e} x ^ {2} + 2 x + 1 = 0 \mathrm {h a s a r e a l} \mathrm {r o o t}.
$$

The existential nature of this assertion can be made more explicit by rewriting it in the form:

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

**Mathematicians have other methods for proving statements of the form $\exists x P(x)$.** For example, one way to prove that the equation $x^3 + 3x + 1 = 0$ has a real root is to note that the curve $y = x^3 + 3x + 1$ is continuous (intuitively, the graph is an unbroken line), that the curve is below the $x$-axis when $x = -1$ and above the $x$-axis when $x = 1$, and hence must (by continuity) cross the $x$-axis somewhere between those two values of $x$. The value of $x$ when it crosses the $x$-axis will be a solution to the given equation. So we have proved that there is a solution without actually finding one. (It takes no small amount of fairly deep mathematics to turn this intuitively simple argument into a totally rigorous proof, but the general idea as I just explained it does work.)


## 2.4.1 Exercises

**The same kind of argument I just outlined to show that the cubic equation $y = x^3 + 3x + 1$ has a real root, can be used to prove the "Wobbly Table Theorem."** Suppose you are sitting in a restaurant at a perfectly square table, with four identical legs, one at each corner. Because the floor is uneven, the table wobbles. One solution is to fold a small piece of paper and insert it under one leg until the table is stable. But there is another solution. Simply by rotating the table you will be able to position it so it does not wobble. Prove this. (WARNING: This is a thinking-outside-the-box question. The solution is simple, but it can take a lot of effort before you find it. This would be an unfair question on a timed exam but is a great puzzle to keep thinking about until you hit upon the right idea.)

**Sometimes it is not immediately obvious that a statement is an existence assertion.** In fact, many mathematical statements that do not look like existence statements on the surface turn out to be precisely that when you work out what they mean. For example, the statement

$$
\sqrt {2} \mathrm {i s r a t i o n a l}
$$

**expresses an existence claim.** You see that when you unpack its meaning and write it in the form

$$
\text {T h e r e} p \text {a n d} q \text {s u c h t h a t} \sqrt {2} = p / q.
$$

**Using the existential quantifier symbol, we might write this as**

$$
\exists p \exists q (\sqrt {2} = p / q)
$$

**This would be fine provided we specified in advance that the variables $p$ and $q$ refer to whole numbers.** Sometimes the context in which we work guarantees that everyone knows what kinds of entities the various symbols refer to. But that is (very) often not the case. So we extend the quantifier notation by specifying the kind of entity under consideration. In this example, we would write

$$
(\exists p \in \mathcal {N}) (\exists q \in \mathcal {N}) (\sqrt {2} = p / q)
$$

**This uses set-theoretic notation that you are probably familiar with, $\mathcal{N}$ denotes the set of natural numbers (i.e., positive whole numbers) and $p \in \mathcal{N}$ means "$p$ is an element (or member) of the set $\mathcal{N}$."** See the appendix for a brief summary of the set theory required for this book.

**Note that I did not write the above formula as $(\exists p, q \in \mathcal{N})(\sqrt{2} = p / q)$.** **You often see experienced mathematicians writing expressions like this, but it is definitely not recommended for beginners.** **Most mathematical statements involve a whole string of quantifiers, and as we'll see, it can get very tricky manipulating the expression in the course of a mathematical argument, so it is safer to stick to the "one variable per quantifier" rule.** I shall definitely do that throughout this book.

**The above statement, $(\exists p \in \mathcal{N})(\exists q \in \mathcal{N})(\sqrt{2} = p / q)$, turns out to be false.** **The number $\sqrt{2}$ is not rational.** I'll give the proof later, but before I do, you might want to see if you can prove it yourself. The argument is only a few lines long, but it involves a clever idea. Chances are, you won't discover it, but if you do, it will definitely make your day! Give it an hour or so.

**Incidentally, one feature you need to get used to in mastering college mathematics, or more generally what I am calling mathematical thinking, is the length of time you may need to spend on one particular detail.** High school mathematics courses (particularly in the US) are generally put together so that most problems can be done in a few minutes, with the goal of covering an extensive curriculum. At college, there is less material to cover, but the aim is to cover it in more depth. That means you have to adjust to the slower pace, with a lot more thinking and less doing. At first, this comes hard, since thinking without seeming to be making progress is initially frustrating. But it's very much like learning to ride a bike. For a long time, you keep falling (or relying on training wheels), and it seems you'll never "get it." Then suddenly, one day, you find you can do it, and you cannot understand why it took so long to get there. But that long period of repeated falling was essential to your body learning how to do it. Training your mind to think mathematically about various kinds of problems is very much like that.

**The remaining piece of language we need to examine and make sure we fully comprehend is the universal quantifier, which asserts that something holds for all $x$.** We use the symbol

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

**As before, this would be fine, provided we specified in advance that the variable $x$ refers to real numbers.** It usually does, of course. But to be sure, we can modify the notation to make it crystal clear and unambiguous:

$$
(\forall x \in \mathcal {R}) (x ^ {2} \geq 0)
$$

**We would read this as "For all real numbers $x$, the square of $x$ is greater than or equal to 0."**

**Most statements in mathematics involve combinations of both kinds of quantifier.** For instance, the assertion that there is no largest natural number requires two quantifiers, thus:

$$
(\forall m \in \mathcal {N}) (\exists n \in \mathcal {N}) (n > m)
$$

**This reads: for all natural numbers $m$ it is the case that there exists a natural number $n$ such that $n$ is greater than $m$.**

**Notice that the order in which quantifiers appear can be of paramount importance.** For example, if we switch the order in the above we get

$$
(\exists n \in \mathcal {N}) (\forall m \in \mathcal {N}) (n > m)
$$

**This asserts that there is a natural number which exceeds all natural numbers —an assertion that is clearly false!**

**Now it should be clear why we need to avoid using language the way the American Melanoma Foundation writer did when crafting that statement.** **One American dies of melanoma almost every hour.** **That sentence has the logical form**

**$\exists A \forall H [A \text{ dies in hour } H]$**

**when what is meant is**

**$\forall H \exists A [A \text{ dies in hour } H]$.**


## 2.4.2 Exercises

1. Express the following as existence assertions. (Feel free to use a mix of symbols and words.)

- (a) The equation $x^{3} = 27$ has a natural number solution.
- (b) $1,000,000$ is not the largest natural number.
- (c) The natural number $n$ is not a prime.

2. Express the following as ‘for all’ assertions (using symbols and words):

- (a) The equation $x^{3} = 28$ does not have a natural number solution.
- (b) $0$ is less than every natural number.
- (c) the natural number $n$ is a prime.

3. Express the following in symbolic form, using quantifiers for people:

- (a) Everybody loves somebody.
- (b) Everyone is tall or short.
- (c) Everyone is tall or everyone is short.
- (d) Nobody is at home.

- (e) If John comes, all the women will leave.
- (f) If a man comes, all the women will leave.

4. Express the following using quantifiers that refer (only) to the sets $\mathcal{R}$ and $\mathcal{N}$:

- (a) The equation $x^{2} + a = 0$ has a real root for any real number $a$.
- (b) The equation $x^{2} + a = 0$ has a real root for any negative real number $a$.
- (c) Every real number is rational.
- (d) There is an irrational number.
- (e) There is no largest irrational number. (This one looks quite complicated.)

5. Let $C$ be the set of all cars, let $D(x)$ mean that $x$ is domestic, and let $M(x)$ mean that $x$ is badly made. Express the following in symbolic form using these symbols:

- (a) All domestic cars are badly made.
- (b) All foreign cars are badly made.
- (c) All badly made cars are domestic.
- (d) There is a domestic car that is not badly made.
- (e) There is a foreign car that is badly made.

6. Express the following sentence symbolically, using only quantifiers for real numbers, logical connectives, the order relation $\langle$, and the symbol $Q(x)$ having the meaning ' $x$ is rational':

There is a rational number between any two unequal real numbers.

- 7. Express the following famous statement (by Abraham Lincoln) using quantifiers for people and times: "You may fool all the people some of the time, you can even fool some of the people all of the time, but you cannot fool all of the people all the time."
- 8. A US newspaper headline read, "A driver is involved in an accident every six seconds." Let $x$ be a variable to denote a driver, $t$ a variable for a six-second interval, and $A(x, t)$ the property that $x$ is in an accident

during interval $t$. Express the headline in logical notation.

**In mathematics (and in everyday life), you often find yourself having to negate a statement involving quantifiers.** **Of course, you can do it simply by putting a negation symbol in front.** **But often that's not enough; you need to produce a positive assertion, not a negative one.** **The examples I'll give should make it clear what I mean by "positive" here, but roughly speaking, a positive statement is one that says what is, rather than what is not.** In practice, a positive statement is one that contains no negation symbol, or else one in which any negation symbols are as far inside the statement as is possible without the resulting expression being unduly cumbersome.

**Let $A(x)$ denote some property of $x$.** (For example, $A(x)$ could say that $x$ is a real root of the equation $x^2 + 2x + 1 = 0$.) I'll show that

$$
\neg [ \forall x A (x) ] \text {i s e q u i v a l e n t t o} \exists x [ \neg A (x) ]
$$

**For example,**

**It is not the case that all motorists run red lights**

**is equivalent to**

**There is a motorist who does not run red lights.**

**With a familiar example like this, the equivalence is obvious.** The general proof requires nothing more than formulating this general understanding in a generic, abstract form. If the following seems at all mysterious, the explanation is undoubtedly that you are simply not used to reasoning in a decontextualized, abstract manner. If you are working through this book in preparation for taking college math courses, then you will need to master abstract reasoning as soon as possible. On the other hand, if your goal is simply to improve your analytic reasoning skills for everyday use, then it is probably enough to replace the abstract symbols by specific, simple examples (as I just did) and work through them, though mastery of abstraction definitely helps everyday reasoning, by highlighting the underlying logic on which all reasoning depends.

**Now for the abstract verification.** We begin by assuming that $\neg [\forall xA(x)]$

**That is, we assume it is not the case that $\forall x A(x)$ is true.** **Well, if it is not the case that all $x$ satisfy $A(x)$, what must happen is that at least one of the $x$ must fail to satisfy $A(x)$.** **In other words, for at least one $x$, $\neg A(x)$ must be true.** **In symbols, this can be written $\exists x[\neg A(x)]$.** **Hence $\neg [\forall x A(x)]$ implies $\exists x[\neg A(x)]$.**

**Now suppose $\exists x[\neg A(x)]$ Thus there will be an $x$ for which $A(x)$ fails.** **Hence $A(x)$ does not hold for every $x$.** **(It fails for the $x$ where it fails!)** **In other words, it is false that $A(x)$ holds for all $x$.** **In symbols, $\neg [\forall xA(x)]$.** **Thus $\exists x[\neg A(x)]$ implies $\neg [\forall xA(x)]$.**

**Taken together, the two implications just established produce the claimed equivalence.**


## 2.4.3 Exercises

- 1. Show that $\neg [\exists xA(x)]$ is equivalent to $\forall x[\neg A(x)]$.
- 2. Give an everyday example to illustrate this equivalence, and verify it by an argument specific to your example.

**Now we are in a position to carry out a proper analysis of our earlier problem about domestic cars, where we want to negate the statement**

**All domestic cars are badly made.**

**Let us formulate this symbolically using the notation of Exercise 2.4.2(5).** If you got part (a) of that question correct, you should have the formulation

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

**If it is not the case that all domestic cars are badly made, then it must be the case that at least one of them fails to be badly made.** Hence, as this argument reverses, the required negation is that at least one domestic car is not badly made.

**The issue discussed above causes problems for enough beginners to warrant some further examples.**

**The first is about natural numbers.** Hence all variables will refer to members of the set $\mathcal{N}$. Let $P(x)$ denotes the property "$x$ is a prime" and $O(x)$ the property "$x$ is odd$).$. Consider the sentence

$$
\forall x [ P (x) \Rightarrow O (x) ]
$$

**This says that all primes are odd, which is false.** (Why? How would you prove that?) The negation of this sentence will have the (positive) form

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

**Thus the $\forall$ becomes a $\exists$ and the $\Rightarrow$ becomes a $\land$.** In words, the negation reads “There is a prime that is not odd,” or more colloquially, “There is an even prime.” This is, of course, true. (Why? How would you prove this?)

**Viewed as a symbolic procedure, what I did above was move the negation symbol successively inside the expression, adjusting the logical connectives appropriately as I did.** **As you will have suspected, it is possible to write down a list of symbol-manipulation rules for doing this kind of thing.** **That would be useful if you wanted to write a computer program to carry out logical reasoning.** But our goal here is to develop mathematical thinking skills. The symbolic examples are merely a way of achieving that, in a manner that is particularly useful for college mathematics students. Thus, I would strongly recommend that you approach every problem in terms of what it means, using its own language.

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

Prove that the statement

> There is an even prime bigger than 2

is false.

For another example, suppose we are talking about people, so $x$ denotes an arbitrary person. Let $P(x)$ be the property of being a player for a certain sports team and $H(x)$ the property of being healthy. Then the sentence

$$
\exists x [ P (x) \land \neg H (x) ]
$$

expresses the claim that there is an unhealthy player. Negating this gives

$$
\forall x [ \neg P (x) \lor H (x) ]
$$

This is a bit unnatural to read in English, but by virtue of the way we defined $\Rightarrow$, it can be rewritten as

$$
\forall x [ P (x) \Rightarrow H (x) ]
$$

and this has the natural reading that “all players are healthy”.

Here is another mathematical example, where the variables denote members of the set $Q$ of all rationals. Consider the sentence

$$
\forall x [ x > 0 \Rightarrow \exists y (x y = 1) ]
$$

This says that every positive rational has a multiplicative inverse (which is true). The negation of this sentence (which will be false) works out as follows.

$$
\begin{array}{l} \neg \forall x [ x > 0 \Rightarrow \exists y (x y = 1) ] \Leftrightarrow \exists x [ x > 0 \land \neg \exists y (x y = 1) ] \\ \Leftrightarrow \exists x [ x > 0 \wedge \forall y (x y \neq 1) ] \\ \end{array}
$$

In words, there is a positive rational $x$ with the property that no $y$ exists such that $xy = 1$, i.e., there is a positive rational with no multiplicative inverse.

The above examples illustrate a feature of quantification that is sufficiently common to warrant systematic development. Associated with any use of quantifiers there is what is known as a domain of quantification: the collection of all objects that the quantifiers refer to. This may be the collection of all real numbers, the collection of all natural numbers, the collection of all complex numbers, or some other collection.

In many cases, the overall context determines the domain. For instance, if we are studying real analysis, then unless otherwise mentioned it may safely be assumed that any quantifier refers to the real numbers. But on occasions there is no alternative but to be quite explicit as to what is the domain under discussion.

To illustrate how it can sometimes be important to specify the domain, consider the mathematical statement

$$
\forall x \exists y (y ^ {2} = x)
$$

This is true for the domain $c$ of complex numbers but not true when the domain is $\mathcal{R}$.

At the risk of confusing you, I should mention that, in practice, mathematicians often omit not only explicit mention of the domain of quantification (leaving it to the context to indicate what the variable denotes), but in the case of universal quantification, all mention of the quantifier at all, writing expressions such as

$$
x \geq 0 \Rightarrow \sqrt {x} \geq 0
$$

when what is meant is

$$
(\forall x \in \mathcal {R}) [ x \geq 0 \Rightarrow \sqrt {x} \geq 0 ]
$$

The former is known as implicit quantification. Although I do not use this convention in this book, implicit quantification is fairly common, so you should be aware of it.

Care has to be exercised when quantifiers are combined with the logical combinators $\Lambda, \vee$, etc.

As an illustration of the various pitfalls that can arise, suppose the domain under discussion is the set of natural numbers. Let $E(x)$ be the statement ' $x$ is even', and let $O(x)$ be the statement ' $x$ is odd'.

The statement

$$
\forall x [ E (x) \lor O (x) ]
$$

says that for every natural number $x$, $x$ is either even or odd (or both). This is clearly true.

On the other hand, the statement

$$
\forall x E (x) \lor \forall x O (x)
$$

is false, since it asserts that either all natural numbers are even or else all natural numbers are odd (or both), whereas in fact neither of these alternatives is the case.

Thus, in general you cannot "move a $\forall x$ inside brackets." More precisely, if you do, you can end up with a very different statement, not equivalent to the original one.

Again, the statement

$$
\exists x [ E (x) \land O (x) ]
$$

is false, since it claims that there is a natural number that is both even and odd, whereas the statement

$$
\exists x E (x) \land \exists x O (x)
$$

claims that there is a natural number that is even and there is a natural number that is odd, which is true.

Thus "moving a $\exists x$ inside brackets" can also lead to a statement that is not equivalent to the original one.

Notice that although the last statement above uses the same variable $x$ in both parts of the conjunction, the two conjuncts operate separately.

You should make sure you fully appreciate the distinction between all the above example statements involving quantifiers.

Very often, in the course of an argument, we use quantifiers that are restricted to a smaller collection than the original domain. For example, in real analysis (where the unspecified domain is usually the set $\mathcal{R}$ of all real numbers) we often need to talk about "all positive numbers" or "all negative numbers", and in number theory (where the unspecified domain is the set $\mathcal{N}$ of all natural numbers) we have quantifiers such as "for all prime numbers", etc.

One way to handle this has been done already. We can modify the quantifier notation, allowing quantifiers of the form

$$
(\forall x \in A), (\exists x \in A)
$$

where $A$ is some subcollection of the domain.

Another way is to specify the objects being quantified within the non-quantifier part of the formula. For example, suppose the domain under discussion is the set of all animals. Thus, any variable $x$ is assumed to denote an animal. Let $L(x)$ mean that " $x$ is a leopard" and let $S(x)$ mean that " $x$ has spots". Then the sentence "All leopards have spots" can be written like this:

$$
\forall x [ L (x) \Rightarrow S (x) ]
$$

In English, this reads literally as: "For all animals $x$, if $x$ is a leopard then $x$ has spots". This is rather cumbersome English, but the mathematical version turns out to be preferable to using a modified quantifier of the form $(\forall x \in \mathcal{L})$ where $\mathcal{L}$ denotes the set of all leopards, since a mathematical argument where quantifiers refer to different domains could easily lead to confusion and error.

Beginners often make the mistake of rendering the original sentence "All leopards have spots" as

$$
\forall x [ L (x) \land S (x) ]
$$

In English, what this says is: "For all animals $x$, $x$ is both a leopard and has spots". Or, smoothing out the English a bit, "All animals are leopards and have spots". This is obviously false; not all animals are leopards, for one thing.

Part of the reason for the confusion is probably the fact that the mathematics goes differently in the case of existential sentences. For example, consider the sentence "There is a horse that has spots". If we let $H(x)$ mean that " $x$ is a horse", then this sentence translates into the mathematical sentence

$$
\exists x [ H (x) \land S (x) ]
$$

Literally: “There is an animal that is both a horse and has spots”.

Contrast this with the sentence

$$
\exists x [ H (x) \Rightarrow S (x) ]
$$

This says that “There is an animal such that if it is a horse, then it has spots”. This does not seem to say anything much, and is certainly not at all the same as saying that there is a spotty horse.

In symbolic terms, the modified quantifier notation

$$
(\forall x \in A) \phi (x)
$$

(where the notation $\varphi(x)$ indicates that $\varphi$ is a statement that involves the variable $x$) may be regarded as an abbreviation for the expression

$$
\forall x [ A (x) \Rightarrow \varphi (x) ]
$$

where $A(x)$ is the property of $x$ being in the collection $A$.

Likewise, the notation

$$
(\exists x \in \mathcal {A}) \phi (x)
$$

may be regarded as an abbreviation for

$$
\exists x [ A (x) \land \varphi (x) ]
$$

In order to negate statements with more than one quantifier, you could start at the outside and work inwards, handling each quantifier in turn. The overall effect is that the negation symbol moves inwards, changing each $\forall$ to an $\exists$ and each $\exists$ to a $\forall$ as it passes. Thus, for example

$$
\begin{array}{l} \lnot [ \forall x \exists y \forall z A (x, y, z) ] \Leftrightarrow \exists x \neg [ \exists y \forall z A (x, y, z) ] \\ \Leftrightarrow \exists x \forall y - [ \forall z A (x, y, z) ] \\ \Leftrightarrow \exists x \forall y \exists z \neg [ A (x, y, z) ] \\ \end{array}
$$

As I said before, however, the purpose of this book is to develop thinking skills, not to learn another collection of cookie-cutter rules you can apply to avoid thinking! Industrial strength mathematics problems often involve fairly complex statements. Mathematicians do sometimes use symbolic manipulations like the above to check their reasoning after the fact, but they invariably do the initial reasoning in terms of what the problem means, not by first translating it to a symbolic form and then cranking a symbolic manipulation procedure. The primary goal of college-level pure mathematics, remember, is understanding. Doing and solving (generally the only goals emphasized at high school) are secondary goals. Applying a set of procedures does not lead to understanding. Thinking about, working with, and eventually (you hope) solving the problem in terms of what it means does.

One further quantifier that is often useful is

there exists a unique $x$ such that ...

The usual notation for this quantifier is

3!

This quantifier can be defined in terms of the other quantifiers, by taking

$$
\exists ! x \varphi (x)
$$

to be an abbreviation for

$$
\exists x [ \varphi (x) \land \forall y [ \varphi (y) \Rightarrow x = y ] ]
$$

(Make sure you understand why this last formula expresses unique existence.)

## 2.4.5 Exercises

1. Translate the following sentences into symbolic form using quantifiers.

In each case the assumed domain is given in parentheses.

- (a) All students like pizza. (All people)
- (b) One of my friends does not have a car. (All people)
- (c) Some elephants do not like muffins. (All animals)
- (d) Every triangle is isosceles. (All geometric figures)
- (e) Some of the students in the class are not here today. (All people)
- (f) Everyone loves somebody. (All people)
- (g) Nobody loves everybody. (all people)
- (h) If a man comes, all the women will leave. (All people)
- (i) All people are tall or short. (All people)
- (j) All people are tall or all people are short. (All people)
- (k) Not all precious stones are beautiful. (All stones)
- (1) Nobody loves me. (All people)
- (m) At least one American snake is poisonous. (All snakes)
- (n) At least one American snake is poisonous. (All animals)

2. Which of the following are true? The domain for each is given in parentheses.

- (a) $\forall x (x + 1 \geq x)$ (Real numbers)
- (b) $\exists x(2x + 3 = 5x + 1)$ (Natural numbers)
- (c) $\exists x(x^{2} + 1 = 2^{x})$ (Real numbers)
- (d) $\exists x(x^{2} = 2)$ (Rational numbers)
- (e) $\exists x(x^{2} = 2)$ (Real numbers)
- (f) $\forall x(x^{3} + 17x^{2} + 6x + 100 \geq 0)$ (Real numbers)
- (g) $\exists x(x^{3} + x^{2} + x + 1 \geq 0)$ (Real numbers)
- (h) $\forall x \exists y (x + y = 0)$ (Real numbers)
- (i) $\exists x \forall y (x + y = 0)$ (Real numbers)
- (j) $\forall x \exists ! y(y = x^2)$ (Real numbers)
- (k) $\forall x \exists ! y(y = x^{2})$ (Natural numbers)
- (1) $\forall x \exists y \forall z (xy = xz)$ (Real numbers)
- (m) $\forall x \exists y \forall z (xy = xz)$ (Prime numbers)

- (n) $\forall x \exists y (x \geq 0 \Rightarrow y^2 = x)$ (Real numbers)
- (0) $\forall x[x < 0 \Rightarrow \exists y(y^2 = x)]$ (Real numbers)
- (p) $\forall x[x < 0 \Rightarrow \exists y(y^2 = x)]$ (Positive real numbers)

- 3. Negate each of the symbolic statements you wrote in Question 1, putting your answers in positive form. Express each negation in natural, idiomatic English.
- 4. Negate each of the statements in Question 2, putting your answers in positive form.
- 5. Negate the following statements and put each answer into positive form:

- (a) $(\forall x\in \mathcal{N})(\exists y\in \mathcal{N})(x + y = 1)$
- (b) $(\forall x > 0)(\exists y < 0)(x + y = 0)$ (where $x, y$ are real number variables)
- (c) $\exists x(\forall \in >0)(-\in <  x <   \in)$ (where $x,y$ are real number variables)
- (d) $(\forall x\in \mathcal{N})(\forall y\in \mathcal{N})(\exists z\in \mathcal{N})(x + y = z^2)$

- 6. Give a negation (in positive form) of the quotation which you met in Exercise 2.4.2(7): "You may fool all the people some of the time, you can even fool some of the people all of the time, but you cannot fool all of the people all the time."
- 7. The standard definition of a real function $f$ being continuous at a point $x = a$ is

$$
(\forall \in > 0) (\exists \delta > 0) (\forall x) [ | x - a | <   \delta \Rightarrow | f (x) - f (a) | <   \in ]
$$

Write down a formal definition for $f$ being discontinuous at $a$. Your definition should be in positive form.
