import mwxml

dump = mwxml.Dump.from_file(open("./test_data/wikidatawiki-20231020-pages-meta-current1.xml-p1p441397"))
i = 0

def page_n(n):
    it = dump.__iter__()
    ref = None
    for i in range(n):
        ref = next(it) # type: ignore

    return ref

def revision_n(page, n):
    it = page.__iter__()
    ref = None
    for i in range(n):
        ref = next(it) # type: ignore
    return ref

page_10 = page_n(10)
revise_10 = revision_n(page_10, 1)
print(dir(revise_10))
print(revise_10.text) # type: ignore


