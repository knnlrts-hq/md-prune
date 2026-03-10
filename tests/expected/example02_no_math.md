**For all the time schools devote to the teaching of mathematics, very little (if any) is spent trying to convey just what the subject is about. Instead, the focus is on learning and applying various procedures to solve math problems. That's a bit like explaining soccer by saying it is executing a series of maneuvers to get the ball into the goal. Both accurately describe various key features, but they miss the what and the why of the big picture.**

# 1.1 More than arithmetic
**Most of the mathematics used in present-day science and engineering is no more than three- or four-hundred years old, much of it less than a century old. Yet the typical high school curriculum comprises mathematics at least that old—some of it over two-thousand years old!**

**The mathematicians of ancient Greece had a particularly high regard for geometry. Indeed, they treated numbers geometrically, as measurements of length, and when they discovered that there were lengths to which their numbers did not correspond (essentially the discovery of irrational numbers), their study of number largely came to a halt.**

**In fact, it was the Greeks who made mathematics into an area of study, not merely a collection of techniques for measuring, counting, and accounting. Around 500 BCE, Thales of Miletus (Miletus is now part of Turkey) introduced the idea that the precisely stated assertions of mathematics could be logically proved by formal arguments. This innovation marked the birth of the theorem, now the bedrock of mathematics. This formal approach by the Greeks culminated in the publication of Euclid's Elements, reputedly the most widely circulated book of all time after the Bible.**

**Virtually nothing from the last three hundred years has found its way into the classroom. Yet most of the mathematics used in today's world was developed in the last two hundred years, let alone the last three hundred!**

**The explosion of mathematical activity that has taken place over the past hundred years or so in particular has been dramatic. At the start of the twentieth century, mathematics could reasonably be regarded as consisting of about twelve distinct subjects: arithmetic, geometry, calculus, and several more. Today, the number of distinct categories is somewhere between sixty and seventy, depending how you count them. Some subjects, like algebra or topology, have split into various subfields; others, such as complexity theory or dynamical systems theory, are completely new areas of study.**

**The dramatic growth in mathematics led in the 1980s to the emergence of a new definition of mathematics as the science of patterns. According to this description, the mathematician identifies and analyzes abstract patterns—numerical patterns, patterns of shape, patterns of motion, patterns of behavior, voting patterns in a population, patterns of repeating chance events, and so on. Those patterns can be either real or imagined, visual or mental, static or dynamic, qualitative or quantitative, utilitarian or recreational. They can arise from the world around us, from the pursuit of science, or from the inner workings of the human mind. Different kinds of patterns give rise to different branches of mathematics.**

# 1.2 Mathematical notation
**One aspect of modern mathematics that is obvious to even the casual observer is the use of abstract notations: algebraic expressions, complicated-looking formulas, and geometric diagrams.** **The mathematicians' reliance on abstract notation is a reflection of the abstract nature of the patterns they study.**

**For example, the commutative law for addition could be written in English as:**

> ***When two numbers are added, their order is not important.**

**These days, mathematics books tend to be awash with symbols, but mathematical notation no more is mathematics than musical notation is music. A page of sheet music represents a piece of music; the music itself is what you get when the notes on the page are sung or performed on a musical instrument. It is in its performance that the music comes alive and becomes part of our experience. The music exists not on the printed page but in our minds. The same is true for mathematics. The symbols on a page are just a representation of the mathematics. When read by a competent performer (in this case, someone trained in mathematics), the symbols on the printed page come alive—the mathematics lives and breathes in the mind of the reader like some abstract symphony.**

**To repeat, the reason for the abstract notation is the abstract nature of the patterns that mathematics helps us identify and study.**

> **The great book of nature can be read only by those who know the language in which it was written. And this language is mathematics.**

**In fact, physics can be accurately described as the universe seen through the lens of mathematics. To take just one example, as a result of applying mathematics to formulate and understand the laws of physics, we now have air travel. When a jet aircraft flies overhead, you can't see anything holding it up. Only with mathematics can we "see" the invisible forces that keep it aloft.** **mathematics makes the invisible visible.**

# 1.3 Modern college-level mathematics
**But during the nineteenth century, as mathematicians tackled problems of ever greater complexity, they began to discover that these earlier intuitions about mathematics were sometimes inadequate to guide their work. Counterintuitive (and occasionally paradoxical) results made them realize that some of the methods they had developed to solve important, real-world problems had consequences they could not explain. The Banach-Tarski Paradox, for example, says you can, in principle, take a sphere and cut it up in such a way that you can reassemble it to form two identical spheres each the same size as the original one. Because the mathematics is correct, the Banach-Tarski result had to be accepted as a fact, even though it defies our imagination.**

**It became clear, then, that mathematics can lead to realms where the only understanding is through the mathematics itself. In order to be confident that we can rely on discoveries made by way of mathematics—but not verifiable by other means—mathematicians turned the methods of mathematics inwards, and used them to examine the subject itself.**

**This introspection led, in the middle of the nineteenth century, to the adoption of a new and different conception of mathematics, where the primary focus was no longer on performing calculations or computing answers, but formulating and understanding abstract concepts and relationships. This was a shift in emphasis from doing to understanding. Mathematical objects were no longer thought of as given primarily by formulas, but rather as carriers of conceptual properties. Proving something was no longer a matter of transforming terms in accordance with rules, but a process of logical deduction from concepts.**

**This revolution—for that is what it amounted to—completely changed the way mathematicians thought of their subject. Yet, for the rest of the world, the shift may as well have not occurred.**

**Then the revolutionary Dirichlet came along and said to forget the formula and concentrate on what the function does in terms of input-output behavior. A function, according to Dirichlet, is any rule that produces new numbers from old. The rule does not have to be specified by an algebraic formula. In fact, there's no reason to restrict your attention to numbers. A function can be any rule that takes objects of one kind and produces new objects from them.**

![[Pasted image 20260309213257.png]]

```python
import numpy as np
import matplotlib.pyplot as plt

# Interval to visualize
xmin, xmax = 0.0, 1.0

# --- Construct many rationals p/q with small denominators ---
max_den = 80  # increase this for more rationals
rats = []
for q in range(1, max_den + 1):
    p = np.arange(int(np.floor(xmin*q)), int(np.ceil(xmax*q)) + 1)
    x = p / q
    x = x[(x >= xmin) & (x <= xmax)]
    rats.append(x)
rat_x = np.unique(np.concatenate(rats))

# --- Sample points to visually represent irrationals ---
# (These are still floating-point rationals, but serve as a stand-in. On a computer, every stored number or float is a rational number (a finite binary fraction). So you _cannot literally_ test “rational vs irrational” from floating-point inputs.)
np.random.seed(0)
irr_x = np.random.uniform(xmin, xmax, 2000)

# --- Plot ---
fig, ax = plt.subplots(figsize=(10, 3))
ax.scatter(rat_x, np.zeros_like(rat_x), s=10, alpha=0.8,
           label=f"Rationals (p/q, q ≤ {max_den}) → 0")
ax.scatter(irr_x, np.ones_like(irr_x), s=6, alpha=0.35,
           label="“Irrationals” (random samples) → 1")

ax.set_xlim(xmin, xmax)
ax.set_ylim(-0.2, 1.2)
ax.set_yticks([0, 1])
ax.set_xlabel("x")
ax.set_ylabel("f(x)")
ax.set_title("Dirichlet function: 0 on rationals, 1 on irrationals")
ax.grid(True, alpha=0.25)
ax.legend(loc="center left", bbox_to_anchor=(1.01, 0.5), frameon=False)
plt.tight_layout()
plt.show()
```

**Mathematicians began to study the properties of such abstract functions, specified not by some formula but by their behavior. For example, does the function have the property that when you present it with different starting values it always produces different answers? (This property is called injectivity or one-to-one.)**

**This abstract, conceptual approach was particularly fruitful in the development of the new subject called real analysis, where mathematicians studied the properties of continuity and differentiability of functions as abstract concepts in their own right. French and German mathematicians developed the "epsilon-delta definitions" of continuity and differentiability that to this day cost each new generation of post-calculus mathematics students so much effort to master.**

- **For every output tolerance $\varepsilon$, there is some input tolerance $\delta$ that works.**

**in the 1850s, Riemann defined a complex function by its property of differentiability, rather than a formula, which he regarded as secondary. The residue classes defined by the famous German mathematician Karl Friedrich Gauss (1777-1855), which you are likely to meet in an algebra course, were a forerunner of the approach—now standard—whereby a mathematical structure is defined as a set endowed with certain operations, whose behaviors are specified by axioms. Taking his lead from Gauss, Dedekind examined the new concepts of ring, field, and ideal, each of which was defined as a collection of objects endowed with certain operations.**

**The revolution may have been quiet, and to a large extent forgotten, but it was complete and far reaching. And it sets the scene for this book, the main aim of which is to provide you with the basic mental tools you will need to enter this new world of modern mathematics (or at least to learn to think mathematically).**

**There was one attempt to introduce the new approach into school classrooms, but it went terribly wrong and soon had to be abandoned. This was the so-called “New Math” movement of the 1960s. What went wrong was that by the time the revolutionaries’ message had made its way from the mathematics departments of the leading universities into the schools, it was badly garbled.**

**To mathematicians before and after the mid-1800s, both calculation and understanding had always been important. The nineteenth century revolution merely shifted the emphasis regarding which of the two the subject was really about and which played the derivative or supporting role. Unfortunately, the message that reached the nation's school teachers in the 1960s was often, "Forget calculation skill, just concentrate on concepts." This ludicrous and ultimately disastrous strategy led the satirist (and mathematician) Tom Lehrer to quip, in his song New Math, "It's the method that's important, never mind if you don't get the right answer."**

**There are educational arguments (which in the absence of hard evidence either way are hotly debated) that say the human mind has to achieve a certain level of mastery of computation with abstract mathematical entities before it is able to reason about their properties.**

# 1.4 Why do you have to learn this stuff?
**It should be clear by now that the nineteenth century shift from a computational view of mathematics to a conceptual one was a change within the professional mathematical community. Their interest, as professionals, was in the very nature of mathematics. For most scientists, engineers, and others who make use of mathematical methods in their daily work, things continued much as before, and that remains the same today. Computation (and getting the right answer) remains just as important as ever, and even more widely used than at any time in history.**

**First, education is not solely about the acquisition of specific tools to use in a subsequent career. As one of the greatest creations of human civilization, mathematics should be taught alongside science, literature, history, and art in order to pass along the jewels of our culture from one generation to the next. We humans are far more than the jobs we do and the careers we pursue. Education is a preparation for life, and only part of that is the mastery of specific work skills.**

**Indeed, in most industries, at almost any level, the mathematical requirements turn out to be higher than is popularly supposed, as many people discover when they look for a job and find their math background lacking. Over many years, we have grown accustomed to the fact that advancement in an industrial society requires a workforce that has mathematical skills. But if you look more closely, those skills fall into two categories. The first category comprises people who, given a mathematical problem (i.e., a problem already formulated in mathematical terms), can find its mathematical solution. The second category comprises people who can take a new problem, say in manufacturing, identify and describe key features of the problem mathematically, and use that mathematical description to analyze the problem in a precise fashion. In the past, there was a huge demand for employees with Type 1 skills, and a small need for Type 2 talent.** **But in today's world, where companies must constantly innovate to stay in business, the demand is shifting toward Type 2 mathematical thinkers—to people who can think outside the mathematical box, not inside it. Now, suddenly, all is not well. There will always be a need for people with mastery of a range of mathematical techniques, who are able to work alone for long periods, deeply focused on a specific mathematical problem, and our education system should support their development. But in the twenty-first century, the greater demand will be for Type 2 ability.**

**A far more important requirement is that they can work well in teams, often cross-disciplinary teams, they can see things in new ways, they can quickly learn and come up to speed on a new technique that seems to be required, and they are very good at adapting old methods to new situations.**

**By the time a college frosh graduates and enters the workforce, many of the specific techniques learned in those college-years are likely to be no longer as important, while new ones are all the rage. The educational focus has to be on learning how to learn.**

**The increasing complexity in mathematics led mathematicians in the nineteenth century to shift (broaden, if you prefer) the focus from computational skills to the underlying, foundational, conceptual thinking ability.**
