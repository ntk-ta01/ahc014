import optuna
import subprocess
import joblib
import time
import statistics

n_cores = -1
n_files = 500


def calc_score_each(seed: int, dmax: int, t0: float, t1: float):
    in_file = open(f"tools/in/{seed:04}.txt", 'r')
    # out_file = open(f"tools/out/{seed:04}.txt", 'w')
    process = subprocess.run(["cargo", "run", "--release", str(dmax), str(t0), str(t1)],
                             stdin=in_file, stdout=subprocess.DEVNULL, encoding='utf-8', stderr=subprocess.PIPE)
    return int(process.stderr.split(':')[-1].strip())


def calc_score(dmax: int, t0: float, t1: float):
    return joblib.Parallel(n_jobs=n_cores)(
        joblib.delayed(calc_score_each)(i, dmax, t0, t1) for i in range(n_files)
    )


def objective(trial: optuna.trial.Trial):
    start = time.time()
    dmax = trial.suggest_int("dmax", 2, 50)
    t0 = trial.suggest_float("t0", 1.0, 50000.0)
    t1 = trial.suggest_float("t1", 0.00001, t0)
    scores = calc_score(dmax, t0, t1)
    print(f"elapsed: {time.time() - start}")
    return statistics.mean(scores)


if __name__ == "__main__":
    study = optuna.create_study(direction="maximize")
    study.optimize(objective, n_trials=3)
    print(study.best_params)
    print(study.best_value)
