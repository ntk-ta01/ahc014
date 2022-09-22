import optuna
import subprocess
import joblib
import statistics

n_cores = -1
n_files = 100


def calc_score_each(seed: int, t0: float, t1: float, back_to_best: int):
    in_file = open(f"tools/in/{seed:04}.txt", 'r')
    # out_file = open(f"tools/out/{seed:04}.txt", 'w')
    process = subprocess.run(["cargo", "run", "--release", str(t0), str(t1), str(back_to_best)],
                             stdin=in_file, stdout=subprocess.DEVNULL, encoding='utf-8', stderr=subprocess.PIPE)
    return int(process.stderr.split(':')[-1].strip())


def calc_score(t0: float, t1: float, back_to_best: int):
    return joblib.Parallel(n_jobs=n_cores)(
        joblib.delayed(calc_score_each)(i, t0, t1, back_to_best) for i in range(n_files)
    )


def objective(trial: optuna.trial.Trial):
    t0 = trial.suggest_float("t0", 6000.0, 9000.0)
    t1 = trial.suggest_float("t1", 5000.0, t0)
    back_to_best = trial.suggest_int("back_to_best", 1000, 30000)
    scores = calc_score(t0, t1, back_to_best)
    return statistics.mean(scores)


if __name__ == "__main__":
    study = optuna.create_study(direction="maximize",
                                storage="sqlite:///ahc014.db",
                                study_name="tune_temp_and_backToBest",
                                load_if_exists=True)
    study.optimize(objective, n_trials=1000)
    print(study.best_params)
    print(study.best_value)
