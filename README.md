# serie5_parsing_fuzzing

## 1 Parsing in Rust 

We are going to parse a CSV file using rust. More precisely, we are going to parse a **movie database**. You will find the dataset here: https://grouplens.org/datasets/movielens/latest/. You can start with the small dataset (that we provide) and then test your code on the larger one. Your code should do the following: 

- Take one argument which is a **movie genre**.
- Compute how many movies are of this genre
- Compute how many movies of this genre there was per year (when non 0). 

We will use `serde` and `csv` to deserialize the data. We provide you with a template doing this. For obtaining the statistics, you might need to use `filter()` and `group_by()`: https://docs.rs/itertools/0.8.0/itertools/trait.Itertools.html#method.group_by Try to make your code run in less than 100ms.

## 2 Web Fuzzing with wfuzz

Fuzzing is usually a very useful tool to find vulnerabilities in a software. In particular, `afl` is a famous tool to fuzz software. In this exercise, we look at a particular type of fuzzing: **web fuzzing**. This allows to search for hidden pages on websites and to fuzz parameters. For this, we will use `wfuzz` and use it to fuzz the following website: http://10.192.72.221:7000/. To access it you will need to be connected to the school’s VPN.

1. Install `wfuzz`: it is in some Linux repositories. Alternatively, you will find it here: https://github.com/xmendez/wfuzz. The following documentation will help you going through the lab: https://wfuzz.readthedocs.io/en/latest/user/getting.html
2. Find the administration `php` page. For this, you can use the `-w` option that lets you pass a dictionary to the fuzzer. The `wordlist/general/common.txt` dictionary can be useful. The part of the URL you want to fuzz should be marked with `FUZZ`. You will have too many results. Use the `--hc` option to filter non interesting values.
3. The administration page access is restricted. Use the fuzzer to bypass the restriction. For this, you need to find a correct GET parameter. We are going to use the **Baseline** technique: we provide a dummy GET parameter and the bruteforce will be executed and compared against this baseline. All pages different from the baseline will be output. For this, use the `--hh BBB` option and write the dummy parameter in curly brackets. You will need a bigger dictionary: `wordlist/general/medium.txt` will do the work.
4. Now that you found the correct GET parameter, you will need to find its value. You can bruteforce integers with the `-z range` if you provide a range. Find a way to filter the replies.
5. Finally, log into PAGS. Bruteforce first the username. You can use the `names` dictionary
6. For the password, you can use the `common_pass` dictionary. The password has to be **encoded in base64**. You can add the encoder using `-z`.

**Warning**: you need to be given the authorization to fuzz a website.