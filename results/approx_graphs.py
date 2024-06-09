import pandas as pd
import matplotlib.pyplot as plt

def draw_graph(quantity, inputname, func):
    # Load the CSV file
    filename = f'{inputname.lower()}_output.txt'
    data = pd.read_csv(filename, header=None, names=['test_name', 'n', 'k', 'time', 'edges', 'alg_name'])

    # Group by algorithm and n, then calculate the average time
    if func == 'Average':
        grouped_data = data.groupby(['alg_name', 'n'])[quantity].mean().reset_index()
    else :
        grouped_data = data.groupby(['alg_name', 'n'])[quantity].min().reset_index()

    variance_data = data.groupby(['n', 'k'])[quantity].var().reset_index()

    # Plot the data
    fig, ax1 = plt.subplots(figsize=(10, 6))

    algorithms = ['Poranen', 'My', 'Schmid', 'Calinescu']
    colors = ['royalblue', 'darkorange', 'forestgreen', 'crimson']
    names = ['Poranen\'s Rule', 'New Rule', 'Schmid\'s Rule', 'Calinescu\'s Rule']
    marker = 'o' if family == 'complete' else '.'
    coeff = 0.001 if quantity == 'time' else 1

    for alg, color, name in zip(algorithms, colors, names):
        alg_data = grouped_data[grouped_data['alg_name'] == alg]
        sorted_data = alg_data.sort_values(by=quantity)
        ax1.plot(alg_data['n'], coeff * sorted_data[quantity], label=name, color=color, linestyle='dotted', marker=marker)

    if quantity == 'edges' and inputname == 'complete':
        unique_n = grouped_data['n'].unique()
        optimal_values = 3 * unique_n - 6
        # minimum_values = 0.388 * optimal_values
        minimum_values2 = 0.393 * optimal_values
        ax1.plot(unique_n, optimal_values, label='Optimal solution', color='deeppink', linestyle='dotted', marker=marker)
        # plt.plot(unique_n, minimum_values, label='7/18 * Optimal solution', color='rebeccapurple', linestyle='dotted', marker=marker)
        ax1.plot(unique_n, minimum_values2, label='13/33 * Optimal solution', color='fuchsia', linestyle='dotted', marker=marker)


    if quantity == 'time':
        ax1.set_yscale('log')

    ax1.set_xlabel('Number of vertices')
   
    if quantity == 'time':
        ax1.set_ylabel(f'{func} {quantity} (ms)')
    else:
        ax1.set_ylabel(f'{func} number of {quantity}')


    ax1.set_title(f'{func} {quantity} comparison on {inputname} graphs')
    ax1.grid(True)
    ax1.set_facecolor('ivory')

    if inputname != 'complete' and quantity == 'edges':
        ax2 = ax1.twinx()
        ax2.plot(variance_data['n'], variance_data[quantity], color='black', linestyle='none', marker='x')
        ax2.set_ylabel(f'Variance of {quantity} for each graph and different algorithms')
        ax1.set_facecolor('ivory')

    lines, labels = ax1.get_legend_handles_labels()
    # lines2, labels2 = ax2.get_legend_handles_labels()
    ax1.legend(lines, labels, loc='upper left')
    

# plt.show()
    plt.savefig(f'{func}_{quantity}_{inputname}_approx.png', dpi=300)

# draw_graph('time')
for quantity in ['edges', 'time']:
    for family in ['complete', '3regular', 'Pareto']:
        draw_graph(quantity, family, 'Average')
