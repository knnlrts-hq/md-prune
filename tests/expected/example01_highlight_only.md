# 1.1 Why Learn This Stuff?

## 1.1.1 Little languages are everywhere
==For every successful general-purpose language, there are a thousand successful niche ones. We used to call them “little languages”, but inflation in the jargon economy led to the name “domain-specific languages”.==

![[2026-02-24 12_49_43-CI-ch01.pdf - Foxit PDF Reader.png]]

## 1.1.2 Languages are great exercise
==Implementing a language is a real test of programming skill. The code is complex and performance critical. You must master recursion, dynamic arrays, trees, graphs, and hash tables. You probably use hash tables at least in your day-to-day programming, but do you really understand them?==

# 1.2 How the Book Is Organized

## 1.2.1 The code
==Many other language books and language implementations use tools like Lex and Yacc, so-called compiler-compilers, that automatically generate some of the source files for an implementation from some higher-level description. There are pros and cons to tools like those, and strong opinions—some might say religious convictions—on both sides. Yacc is a tool that takes in a grammar file and produces a source file for a compiler, so it’s sort of like a “compiler” that outputs a compiler, which is where we get the term “compiler-compiler”.==

![[8e3146de2bad49f6c3d47e14ad9624fdf9f6f8b9c9ed693f912c4bbf52d91ab8.jpg]]

## 1.2.2 Snippets
```java
default: 
    if (isDigit(c)) { 
        number(); 
    } else { 
        Lox.error(line, "Unexpected character."); 
    } 
    break; 
```

# 1.3 The First Interpreter
==Java is a great language for this. It’s high level enough that we don’t get overwhelmed by fiddly implementation details, but it’s still pretty explicit. Unlike in scripting languages, there tends to be less complex machinery hiding under the hood, and you’ve got static types to see what data structures you’re working with. I also chose Java specifically because it is an object-oriented language.==

==Object-oriented languages are ubiquitous, and the tools and compilers for a language are often written in the same language. A compiler reads files in one language, translates them, and outputs files in another language. You can implement a compiler in any language, including the same language it compiles, a process called self-hosting. If your new compiler is written in its own language, there’s an obvious problem at first: how do you compile it before your compiler exists? You can’t compile your compiler using itself yet, but if you have another compiler for your language written in some other language, you use that one to compile your compiler once. Now you can use the compiled version of your own compiler to compile future versions of itself, and you can discard the original one compiled from the other compiler. This is called bootstrapping==

![[3c3f9d9d40208f2d8db0495ffdc2318160781adb21ba045e6ae3f7c1db5fb43b.jpg]]

# 1.4 The Second Interpreter
==In our C interpreter, `clox`, we are forced to implement for ourselves all the things Java gave us for free. We’ll write our own dynamic array and hash table. We’ll decide how objects are represented in memory, and build a garbage collector to reclaim them.==

# CHALLENGES
	```java
	public static void main(String[] args) {
	  System.out.println("Hello!");
	  try {
	    Thread.sleep(5000);
	  } catch (InterruptedException e) {
	    e.printStackTrace();
	  }
	}
	```

	```c
	#ifndef DLIST_H
	#define DLIST_H
	
	#include <stdbool.h>
	#include <stddef.h>
	
	typedef struct Node {
	    char *value;
	    struct Node *prev;
	    struct Node *next;
	} Node;
	
	typedef struct {
	    Node *head;
	    Node *tail;
	    size_t size;
	} DList;
	
	void dlist_init(DList *list);
	Node *dlist_insert_front(DList *list, const char *s);
	Node *dlist_insert_back(DList *list, const char *s);
	Node *dlist_find(const DList *list, const char *s);
	bool dlist_delete(DList *list, const char *s);   // deletes first match
	void dlist_destroy(DList *list);
	void dlist_print(const DList *list);
	
	#endif
	```

	```c
	#include "dlist.h"

	#include <stdio.h>
	#include <stdlib.h>
	#include <string.h>
	
	static char *xstrdup(const char *s) {
	    size_t len = strlen(s);
	    char *copy = malloc(len + 1);
	    if (copy == NULL) {
	        perror("malloc");
	        exit(EXIT_FAILURE);
	    }
	    memcpy(copy, s, len + 1);
	    return copy;
	}
	
	void dlist_init(DList *list) {
	    list->head = NULL;
	    list->tail = NULL;
	    list->size = 0;
	}
	
	Node *dlist_insert_front(DList *list, const char *s) {
	    Node *node = malloc(sizeof(*node));
	    if (node == NULL) {
	        perror("malloc");
	        exit(EXIT_FAILURE);
	    }
	
	    node->value = xstrdup(s);
	    node->prev = NULL;
	    node->next = list->head;
	
	    if (list->head != NULL) {
	        list->head->prev = node;
	    } else {
	        list->tail = node;
	    }
	
	    list->head = node;
	    list->size++;
	    return node;
	}
	
	Node *dlist_insert_back(DList *list, const char *s) {
	    Node *node = malloc(sizeof(*node));
	    if (node == NULL) {
	        perror("malloc");
	        exit(EXIT_FAILURE);
	    }
	
	    node->value = xstrdup(s);
	    node->prev = list->tail;
	    node->next = NULL;
	
	    if (list->tail != NULL) {
	        list->tail->next = node;
	    } else {
	        list->head = node;
	    }
	
	    list->tail = node;
	    list->size++;
	    return node;
	}
	
	Node *dlist_find(const DList *list, const char *s) {
	    for (Node *cur = list->head; cur != NULL; cur = cur->next) {
	        if (strcmp(cur->value, s) == 0) {
	            return cur;
	        }
	    }
	    return NULL;
	}
	
	bool dlist_delete(DList *list, const char *s) {
	    Node *node = dlist_find(list, s);
	    if (node == NULL) {
	        return false;
	    }
	
	    if (node->prev != NULL) {
	        node->prev->next = node->next;
	    } else {
	        list->head = node->next;
	    }
	
	    if (node->next != NULL) {
	        node->next->prev = node->prev;
	    } else {
	        list->tail = node->prev;
	    }
	
	    free(node->value);
	    free(node);
	    list->size--;
	    return true;
	}
	
	void dlist_destroy(DList *list) {
	    Node *cur = list->head;
	    while (cur != NULL) {
	        Node *next = cur->next;
	        free(cur->value);
	        free(cur);
	        cur = next;
	    }
	
	    list->head = NULL;
	    list->tail = NULL;
	    list->size = 0;
	}
	
	void dlist_print(const DList *list) {
	    printf("[");
	    for (Node *cur = list->head; cur != NULL; cur = cur->next) {
	        printf("%s%s", cur->value, cur->next ? ", " : "");
	    }
	    printf("]\n");
	}
	```

	```c
	#include "dlist.h"
	
	#include <assert.h>
	#include <stdio.h>
	#include <string.h>
	
	static void assert_list_shape(const DList *list, const char *const expected[], size_t count) {
	    assert(list->size == count);
	
	    size_t i = 0;
	    Node *prev = NULL;
	
	    for (Node *cur = list->head; cur != NULL; cur = cur->next) {
	        assert(i < count);
	        assert(strcmp(cur->value, expected[i]) == 0);
	        assert(cur->prev == prev);
	        prev = cur;
	        i++;
	    }
	
	    assert(i == count);
	
	    if (count == 0) {
	        assert(list->head == NULL);
	        assert(list->tail == NULL);
	    } else {
	        assert(list->tail != NULL);
	        assert(strcmp(list->tail->value, expected[count - 1]) == 0);
	        assert(list->tail->next == NULL);
	    }
	}
	
	static void test_basic_operations(void) {
	    DList list;
	    dlist_init(&list);
	
	    dlist_insert_back(&list, "alpha");
	    dlist_insert_back(&list, "beta");
	    dlist_insert_back(&list, "gamma");
	    dlist_insert_back(&list, "delta");
	
	    const char *all[] = {"alpha", "beta", "gamma", "delta"};
	    assert_list_shape(&list, all, 4);
	
	    Node *found = dlist_find(&list, "gamma");
	    assert(found != NULL);
	    assert(strcmp(found->value, "gamma") == 0);
	    assert(found->prev != NULL && strcmp(found->prev->value, "beta") == 0);
	    assert(found->next != NULL && strcmp(found->next->value, "delta") == 0);
	
	    assert(dlist_delete(&list, "alpha") == true);  // head
	    const char *after_head[] = {"beta", "gamma", "delta"};
	    assert_list_shape(&list, after_head, 3);
	
	    assert(dlist_delete(&list, "gamma") == true);  // middle
	    const char *after_middle[] = {"beta", "delta"};
	    assert_list_shape(&list, after_middle, 2);
	
	    assert(dlist_delete(&list, "delta") == true);  // tail
	    const char *after_tail[] = {"beta"};
	    assert_list_shape(&list, after_tail, 1);
	
	    assert(dlist_delete(&list, "does-not-exist") == false);
	    assert_list_shape(&list, after_tail, 1);
	
	    dlist_destroy(&list);
	    assert_list_shape(&list, NULL, 0);
	}
	
	static void test_duplicates(void) {
	    DList list;
	    dlist_init(&list);
	
	    dlist_insert_back(&list, "x");
	    dlist_insert_back(&list, "y");
	    dlist_insert_back(&list, "x");
	
	    const char *initial[] = {"x", "y", "x"};
	    assert_list_shape(&list, initial, 3);
	
	    assert(dlist_delete(&list, "x") == true);  // deletes first match
	    const char *after_first[] = {"y", "x"};
	    assert_list_shape(&list, after_first, 2);
	
	    assert(dlist_delete(&list, "x") == true);
	    const char *after_second[] = {"y"};
	    assert_list_shape(&list, after_second, 1);
	
	    dlist_destroy(&list);
	}
	
	int main(void) {
	    test_basic_operations();
	    test_duplicates();
	    puts("All tests passed.");
	    return 0;
	}
	```
