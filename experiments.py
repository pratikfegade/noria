import subprocess

# exp setup
nusers = [10000]
nposts = [10]
nclasses = [100]


for users in nusers:
    for classes in nclasses:
        for posts in nposts:
            postcount = posts * classes
            cmd = "cargo run --release --bin=piazza --manifest-path noria-benchmarks/Cargo.toml -- -l {} -p {} -u {} -c {} --policies noria-benchmarks/piazza/basic-policies.json > exp-{}u-{}ppc-{}c.txt".format(users, postcount, users, classes, users, posts, classes)
            print("CMD: {}".format(cmd))
            subprocess.call(cmd, shell=True)
