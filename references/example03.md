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

