import pandas as pd
import matplotlib.pyplot as plt

def draw_graph(quantity, family, func):
    # Load the CSV file
    filename = 'new_exact_regular_output2.txt'
    data = pd.read_csv(filename, header=None, names=['test_name', 'n', 'k', 'time', 'edges', 'alg_name'])

    # Filter out FacialWalks data
    facial_walks_data = data[data['alg_name'] == 'FacialWalks'].copy()

    # Create a new DataFrame to store the ratios
    ratio_data = data.copy()

    # Calculate the ratio for each algorithm relative to FacialWalks
    for alg in data['alg_name'].unique():
        if alg != 'FacialWalks':
            alg_data = data[data['alg_name'] == alg]
            # Merge with FacialWalks data on 'n'
            merged_data = pd.merge(alg_data, facial_walks_data, on=['n'], suffixes=('', '_facial'))
            merged_data[f'{quantity}_ratio'] = merged_data[f'{quantity}'] / merged_data[f'{quantity}_facial']
            ratio_data.loc[ratio_data['alg_name'] == alg, quantity] = merged_data[f'{quantity}_ratio']

    # Filter out FacialWalks data from the ratio_data
    ratio_data = ratio_data[ratio_data['alg_name'] != 'FacialWalks']

    # Group by algorithm and n, then calculate the average or minimum of the ratio
    if func == 'Average':
        grouped_data = ratio_data.groupby(['alg_name', 'n'])[quantity].mean().reset_index()
    else:
        grouped_data = ratio_data.groupby(['alg_name', 'n'])[quantity].min().reset_index()

    # Plot the data
    fig, ax1 = plt.subplots(figsize=(10, 6))

    algorithms = ['Poranen', 'My', 'Schmid', 'Calinescu']
    colors = ['royalblue', 'darkorange', 'forestgreen', 'crimson']
    names = [r'$\mathsf{CA}_P$', r'$\mathsf{CA}_W$', r'$\mathsf{CA}_S$', r'$\mathsf{CA}$']
    marker = 'o' if family == 'complete' else '.'

    for alg, color, name in zip(algorithms, colors, names):
        alg_data = grouped_data[grouped_data['alg_name'] == alg]
        sorted_data = alg_data.sort_values(by=quantity)
        ax1.plot(alg_data['n'], sorted_data[quantity], label=name, color=color, linestyle='dotted', marker=marker)

    ax1.set_xlabel('Number of vertices')

    if quantity == 'time':
        ax1.set_ylabel(f'{func} {quantity} (ms)')
        ax1.set_yscale('log')
    else:
        ax1.set_ylabel(f'{func} approximation ratio')

    ax1.set_title(f'{func} approximation ratio on {family} graphs')
    ax1.grid(True)
    ax1.set_facecolor('ivory')
    ax1.axhline(y=4/9, color='slategrey', linestyle='--', label=r'$\frac{4}{9}$')

    lines, labels = ax1.get_legend_handles_labels()
    ax1.legend(lines, labels, loc='upper left')

    plt.savefig(f'{func}_{quantity}_{family}_ratio.png', dpi=300)

for quantity in ['edges', 'time']:
    draw_graph(quantity, '3-regular', 'Average')
