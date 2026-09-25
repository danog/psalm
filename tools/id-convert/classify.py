import json,collections,re,sys
S=sys.argv[1]
p=[i for i in json.load(open(S+"/pristine.json")) if i["severity"]=="error"]
c=[i for i in json.load(open(S+"/tx-final.json")) if i["severity"]=="error"]
def pm(m, t=''):
    m = re.sub(r"/\S*?/src/psalm/(\S*?):\d+:\d+:-:closure", r"src/psalm/\1:closure", m)
    m = m.replace('/root/idconv/master-ref/', '/root/idconv/master/')
    # messages whose text drifts with the types / sizes of the code, not with what is wrong
    if t in ('ComplexMethod', 'RiskyTruthyFalsyComparison'):
        m = ''
    return m
pk=collections.Counter((i["file_name"],i["type"],pm(i["message"],i["type"])) for i in p)
new=[]
for i in c:
    k=(i["file_name"],i["type"],pm(i["message"],i["type"]))
    if pk[k]>0: pk[k]-=1
    else: new.append(i)
json.dump(new,open(S+"/new.json","w"))
print("pristine",len(p),"converted",len(c),"new",len(new))
def norm(m):
    m=re.sub(r"\$[\w>\-\[\]'\"]+","$X",m)
    m=re.sub(r"Argument \d+ of \S+","Arg of F",m)
    m=re.sub(r"\d{6,}","ID",m)
    m=re.sub(r"Psalm\\[\w\\]+","P",m)
    m=re.sub(r"for [\w\\:]+","for F",m)
    return m[:170]
for t,n in collections.Counter(i["type"] for i in new).most_common():
    xs=[i for i in new if i["type"]==t]
    print("=====",t,n)
    cc=collections.Counter(norm(i["message"]) for i in xs)
    ex={}
    for i in xs: ex.setdefault(norm(i["message"]),i)
    for k,m in cc.most_common(6):
        e=ex[k]
        print("  %4d %s" % (m,k))
        print("        e.g. %s:%d  %r" % (e["file_name"], e["line_from"], e["selected_text"][:100]))
