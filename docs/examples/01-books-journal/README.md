# A journal for books

Let's suppose you're a very enthusiastic reader. You buy books all the time and some of them you sell back second-hand. Since you spend quite a bit of money on this hobby, you decided to set a yearly budget. All the money you spend buying books is deducted from the budget, and all the money you earn selling is added back.

Being the geek you are, you get an interesting idea. You decide to track these expenses in a few different ways. Apart from being able to tell how much budget you have left at any given point, you want to know the following:

- How much did you spend on new books?
- How much did you spend on second-hand books?
- How much did you spend on books about software?
- How many books did you buy?
- How many books did you sell?
- How much did you earn selling?

That's a lot of things to track. Fortunately, boki can help you manage it. It does this in two important ways:

1. It allows you to record all the necessary information in a way that's easy for humans to read and edit.
2. It can export this information into a format that's easy for machines to read and edit (JSON).

See `./books.boki` for an example of how a journal might look. Note that boki, by itself, does not do any aggregation or calculations. The idea is to export your journal to JSON, and then use something like `jq` or Python to analyze that.
